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
    seconds = Math.abs(seconds);
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
