import { vi } from "vitest";

// `vuestic-ui` pulls in the whole UI library; the composables only use
// `useToast`, so stub it out to keep unit tests fast and free of a plugin
// context that only exists inside a mounted Vue app.
vi.mock("vuestic-ui", () => ({
    useToast: () => ({
        init: vi.fn(),
    }),
}));
