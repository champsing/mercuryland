import { describe, expect, it, vi } from "vitest";
import { notifyConnectionFailure } from "../connectionNotice";

const { init } = vi.hoisted(() => ({ init: vi.fn() }));

vi.mock("vuestic-ui", () => ({
    useToast: () => ({ init }),
}));

describe("notifyConnectionFailure", () => {
    it("shows a top-right error toast", () => {
        notifyConnectionFailure();

        expect(init).toHaveBeenCalledWith(
            expect.objectContaining({
                message: "無法連接到伺服器",
                position: "top-right",
                color: "danger",
            }),
        );
    });
});
