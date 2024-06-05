export function truncateText(str: string, len: number) {
    if (str.length > len) {
        return str.substring(0, len - 2) + "...";
    } else {
        return str;
    }
}

export function openLink(link: string) {
    window.open(link);
}

// export function openDate(date: string) {
//     //switch tab
//     filterData.begTs = date;
//     filterData.endTs = date;
// }