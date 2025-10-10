import { useToast } from "vuestic-ui";

export const BASE_URL = import.meta.env.PROD
    ? "https://api.mercuryland.pp.ua"
    : "http://127.0.0.1:8080";

export function openLinks(links: Array<string>) {
    for (let i = 0; i < links.length; i++) {
        window.open(links[i], "_blank", "noopener noreferrer");
    }
}

export function formatDate(date: Date): string {
    let year = date.getFullYear();
    let month = date.getMonth() + 1;
    let day = date.getDate();
    if (month >= 10) {
        if (day >= 10) return `${year}-${month}-${day}`;
        else return `${year}-${month}-0${day}`;
    } else {
        if (day >= 10) return `${year}-0${month}-${day}`;
        else return `${year}-0${month}-0${day}`;
    }
}

export function parseDate(text): Date {
    const [year, month, day] = text.split("-");
    return new Date(year, month - 1, day);
}

export function backToTop() {
    window.scrollTo({ top: 0, behavior: "smooth" });
}

export async function copyToClipboard(text: string) {
    try {
        await navigator.clipboard.writeText(text);
        useToast().init({
            duration: 2000,
            message: "已複製",
        });
    } catch (err) {
        console.error("Failed to copy: ", err);
    }
}
