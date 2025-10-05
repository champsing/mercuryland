export interface VodItem {
    id: number;
    date: string;
    link: string;
    title: string;
    tags: string[];
    duration: string;
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
