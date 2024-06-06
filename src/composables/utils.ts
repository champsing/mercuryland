export function truncateText(str: string, len: number) {
    if (str == "3̧̮͎ͧ̄̅̔͑͘d͈̦͔̎̄͋ͮ̓͋a̸̧̡͙̗̻̣̩̓v͕͈̯̟̳̤̀̾ͦ̈Ṅ͙͙̬̝̝̈̀9̴̭̮͈̍ͥ̓͛Gͨ͂ͫ̅͟r̩̟͎̺̜̝͙̓̓ͩC͖̣̓͐̅͛̐̈́͢͞D͚̲̳̈͒̋͑ͧ̚q͓̖ͪ͌̚c͕͓̻̬͇̊̓͟ͅȩ̥̮͖͔͉̉̊́͛b̦̜̦̏͟7̷̛͍͔̅ͤm̨̤̹ͤ̑ͤ͡") {
        return str
    } else if (str.length > len) {
        return str.substring(0, len - 2) + "...";
    } else {
        return str;
    }
}

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
