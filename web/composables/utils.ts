import { useToast } from "vuestic-ui";

export const BASE_URL = import.meta.env.PROD
  ? "https://api.mercuryland.pp.ua"
  : "http://127.0.0.1:8080";

export function openLinks(links: Array<string>) {
  for (let i = 0; i < links.length; i++) {
    window.open(links[i], "_blank", "noopener noreferrer");
  }
}

export function ofId<T extends { id: number }>(data: T[], id: number): T {
  for (let i = 0; i < data.length; i++) {
    if (data[i].id == id) {
      return data[i];
    }
  }
  return null;
}

// parse HH:MM:SS format into seconds
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

export function backToTop() {
  window.scrollTo({ top: 0, behavior: "smooth" });
}

// export function interleave<T>(arr: T[], x: T): T[] {
//     return arr.flatMap((e) => [e, x]).slice(0, -1);
// }

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

// export function ccMix(text: string): string {
//     return Array.from(text).map((c) => {
//         if (Math.random() < 0.5) {
//             return c
//         } else {
//             return ccConvertText(c)
//         }
//     }).join("")
// }

export function truncateString(str, num) {
  if (str.length > num) {
    return str.slice(0, num) + "...";
  } else {
    return str;
  }
}
