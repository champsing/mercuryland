import { useToast } from "vuestic-ui";

/** Tell the user the API is unreachable; the toast sits in the top-right. */
export function notifyConnectionFailure(): void {
    useToast().init({
        message: "無法連接到伺服器",
        position: "top-right",
        color: "danger",
        duration: 3000,
        closeable: true,
    });
}
