import type { InjectionKey, Ref } from "vue";

/**
 * Measured height of the app footer in CSS pixels, provided by `App.vue`.
 *
 * The footer is a single row on desktop and two rows on mobile, so layouts
 * that size themselves against the viewport need the real height instead of
 * the hard-coded desktop value.
 */
export const APP_FOOTER_HEIGHT: InjectionKey<Ref<number>> =
    Symbol("app-footer-height");
