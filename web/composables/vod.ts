export interface VodItem {
    id: number;
    date: string;
    link: string;
    title: string;
    tags: string[];
    duration: string;
}

export interface CalculationEntry {
    date: string;
    offset: number;
    previous: number;
    reason: string;
    divider: boolean;
}

export interface VodTimeScheduleItem {
    date: string;
    duration: string;
}

export interface VodTimeOverride extends VodTimeScheduleItem {
    reason: string;
}

export interface VodTimeSchedule {
    initial: string;
    schedule: VodTimeScheduleItem[];
    override: VodTimeOverride[];
}

export interface CalculationGroup {
    start: string;
    previous: number;
    entries: CalculationEntry[];
}

/**
 * Groups entries into weeks. Each weekly 計劃 entry opens a new group; every
 * entry after it (streams, penalties, purchases) belongs to that week. Entries
 * that precede the first 計劃 (a partially filtered week) form their own group.
 */
export function groupCalculationEntries(
    entries: CalculationEntry[],
): CalculationGroup[] {
    const groups: CalculationGroup[] = [];

    for (const entry of entries) {
        const current = groups[groups.length - 1];
        if (!current || entry.divider) {
            groups.push({
                start: entry.date,
                previous: entry.previous,
                entries: [entry],
            });
        } else {
            current.entries.push(entry);
        }
    }

    return groups;
}

export interface CalculationMonth {
    key: string;
    label: string;
    groups: CalculationGroup[];
}

/** Groups weeks into months, labelled for display, e.g. "2026年10月". */
export function groupCalculationMonths(
    groups: CalculationGroup[],
): CalculationMonth[] {
    const months: CalculationMonth[] = [];

    for (const group of groups) {
        const key = group.start.slice(0, 7);
        const current = months[months.length - 1];
        if (current && current.key === key) {
            current.groups.push(group);
        } else {
            const [year, month] = key.split("-");
            months.push({
                key,
                label: `${year}年${Number(month)}月`,
                groups: [group],
            });
        }
    }

    return months;
}

/** ISO-8601 week number of the week containing the given YYYY-MM-DD date. */
export function isoWeekNumber(date: string): number {
    const target = new Date(date + "T00:00:00Z");
    const dayNumber = target.getUTCDay() || 7;
    target.setUTCDate(target.getUTCDate() + 4 - dayNumber);
    const yearStart = new Date(Date.UTC(target.getUTCFullYear(), 0, 1));

    return Math.ceil(
        ((target.getTime() - yearStart.getTime()) / 86400000 + 1) / 7,
    );
}

export function parseHMS(hms: string): number {
    let [s = 0, m = 0, h = 0] = hms.split(":").reverse();
    return +h * 3600 + +m * 60 + +s;
}

export function formatHMS(seconds: number) {
    // Averages and medians can land between seconds.
    seconds = Math.round(Math.abs(seconds));
    let s = seconds % 60;
    let m = Math.floor(seconds / 60) % 60;
    let h = Math.floor(seconds / 3600);

    return (
        h.toString().padStart(2, "0") +
        ":" +
        m.toString().padStart(2, "0") +
        ":" +
        s.toString().padStart(2, "0")
    );
}

/**
 * Builds the weekly ledger behind the remaining-hours balance.
 *
 * Every 計劃 entry opens a week and adds the planned hours. Live streams
 * subtract their duration, while overrides (penalties and paid overtime) add
 * theirs. Entries are emitted chronologically so grouping by week and month
 * can rely on adjacent dates.
 */
export function calculateVodTime(
    vodItems: VodItem[],
    schedule: VodTimeSchedule,
    now: Date = new Date(),
): CalculationEntry[] {
    const entries: CalculationEntry[] = [];

    const weeks = schedule.schedule;
    const overrides = schedule.override
        .slice()
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

    let s = 0;
    let o = 0;
    let v = 0;

    const date = new Date(
        Math.min(
            (weeks.length === 0 ? now : new Date(weeks[0].date)).getTime(),
            (overrides.length === 0
                ? now
                : new Date(overrides[0].date)
            ).getTime(),
        ),
    );

    const streams = vodItems
        .filter((item) => item.date > date.toISOString().slice(0, 10))
        .slice()
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

    let previous = parseHMS(schedule.initial);

    while (date < now) {
        if (weeks.length > 0) {
            if (
                s + 1 < weeks.length &&
                weeks[s + 1].date >= date.toISOString().slice(0, 10)
            ) {
                s = s + 1;
            }

            entries.push({
                date: date.toISOString().slice(0, 10),
                offset: parseHMS(weeks[s].duration),
                previous: previous,
                reason: "計劃",
                divider: true,
            });
            previous = previous + entries[entries.length - 1].offset;
        }

        date.setDate(date.getDate() + 7);

        // Collect this week's live streams and overrides into one list so the
        // display stays chronological regardless of which source comes first.
        const weekEntries: CalculationEntry[] = [];

        while (v < streams.length && new Date(streams[v].date) < date) {
            weekEntries.push({
                date: streams[v].date,
                offset: -parseHMS(streams[v].duration),
                previous: previous,
                reason: "直播",
                divider: false,
            });

            v = v + 1;
        }

        while (o < overrides.length && new Date(overrides[o].date) < date) {
            weekEntries.push({
                date: overrides[o].date,
                offset: parseHMS(overrides[o].duration),
                previous: previous,
                reason: overrides[o].reason,
                divider: false,
            });

            o = o + 1;
        }

        weekEntries.sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

        for (const entry of weekEntries) {
            entry.previous = previous;
            previous = previous + entry.offset;
            entries.push(entry);
        }
    }

    return entries;
}

export interface CalculationSummary {
    /** Opening balance plus every positive entry (計劃, 懲罰, 課金). */
    owed: number;
    /** Hours already streamed (直播 entries). */
    paid: number;
    /** owed − paid */
    remaining: number;
}

/** Splits the ledger into the two sides of the remaining-hours equation. */
export function summarizeCalculation(
    entries: CalculationEntry[],
): CalculationSummary {
    if (entries.length === 0) {
        return { owed: 0, paid: 0, remaining: 0 };
    }

    let owed = entries[0].previous;
    let paid = 0;

    for (const entry of entries) {
        if (entry.offset >= 0) {
            owed += entry.offset;
        } else {
            paid -= entry.offset;
        }
    }

    return { owed, paid, remaining: owed - paid };
}

export interface DurationSummary {
    count: number;
    total: number;
    average: number;
    median: number;
    longest: number;
    shortest: number;
}

/** Descriptive statistics for a set of stream durations, in seconds. */
export function summarizeDurations(seconds: number[]): DurationSummary {
    if (seconds.length === 0) {
        return {
            count: 0,
            total: 0,
            average: 0,
            median: 0,
            longest: 0,
            shortest: 0,
        };
    }

    const sorted = seconds.slice().sort((lhs, rhs) => lhs - rhs);
    const total = seconds.reduce((sum, value) => sum + value, 0);
    const middle = Math.floor(sorted.length / 2);
    const median =
        sorted.length % 2 === 0
            ? (sorted[middle - 1] + sorted[middle]) / 2
            : sorted[middle];

    return {
        count: seconds.length,
        total,
        average: total / seconds.length,
        median,
        longest: sorted[sorted.length - 1],
        shortest: sorted[0],
    };
}

/**
 * Counts durations into `edges.length + 1` buckets. A value equal to an edge
 * falls into the bucket above it, so the buckets read as `< 1 小時`,
 * `1–2 小時`, … without overlapping.
 */
export function durationHistogram(
    seconds: number[],
    edges: number[],
): number[] {
    const counts = new Array<number>(edges.length + 1).fill(0);

    for (const value of seconds) {
        const index = edges.findIndex((edge) => value < edge);
        counts[index === -1 ? edges.length : index] += 1;
    }

    return counts;
}

/**
 * Trailing moving average. The first values average only the points available
 * before the window is full, so the line starts at the first stream.
 */
export function movingAverage(values: number[], window: number): number[] {
    if (window <= 0) return [];

    return values.map((_, index) => {
        const start = Math.max(0, index - window + 1);
        const slice = values.slice(start, index + 1);
        return slice.reduce((sum, value) => sum + value, 0) / slice.length;
    });
}
