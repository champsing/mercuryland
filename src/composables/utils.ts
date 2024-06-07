export function openLink(link: string) {
    window.open(link);
}

export function openLinks(links: Array<string>) {
    for (let i = 0; i < links.length; i++) {
        openLink(links[i]);
    }
}

// parse HH:MM:SS format into seconds
export function parseHMS(hms: string): number {
    let [s = 0, m = 0, h = 0] = hms.split(":").reverse();
    return +h * 3600 + +m * 60 + +s;
}

export function formatHMS(seconds: number) {
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
