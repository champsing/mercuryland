export function openLink(link: string) {
    window.open(link);
}

export function ofId<T extends { id: number }>(data: T[], id: number): T {
    for (let i = 0; i < data.length; i++) {
        if (data[i].id == id) {
            return data[i];
        }
    }
    return null;
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

export function remainingX(el: Element) {
    return (
        document.documentElement.clientWidth -
        el.getBoundingClientRect().left +
        window.screenX
    );
}

export function remainingY(el) {

    return (
        document.documentElement.clientHeight -
        el.getBoundingClientRect().top +
        window.scrollY
    );
}

export function interleave<T>(arr: T[], x: T): T[] {
    return arr.flatMap((e) => [e, x]).slice(0, -1);
}
