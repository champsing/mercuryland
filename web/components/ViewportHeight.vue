<script setup lang="ts">
import {
    APP_FOOTER_COPYRIGHT_HEIGHT,
    APP_FOOTER_HEIGHT,
} from "@/composables/appFooter";
import { availableHeight, FOOTNOTE_HEIGHT } from "@/composables/viewportHeight";
import {
    useElementBounding,
    useMediaQuery,
    useResizeObserver,
    useWindowSize,
} from "@vueuse/core";
import { computed, inject, ref } from "vue";

const props = defineProps<{
    /** Height added back so collapsible panels above do not shrink the area. */
    extraHeight?: number;
    /** Page scroll container; when it actually scrolls, measure against it. */
    scrollContainer?: HTMLElement | null;
    /** Smallest usable area height; defaults to the shared minimum. */
    minHeight?: number;
    /**
     * Leave the wrapper at its natural height instead of pinning it to the
     * viewport. Used where the page itself is the scroller (mobile lists).
     */
    autoHeight?: boolean;
    /**
     * On phones, stop at the footer's copyright line and leave the four action
     * buttons below the fold, so the list gets the maximum visible height.
     */
    footerCopyrightOnly?: boolean;
}>();

const vh = useWindowSize().height;
const isPhone = useMediaQuery("(max-width: 767px)");
const appFooterHeight = inject(APP_FOOTER_HEIGHT, null);
const appFooterCopyrightHeight = inject(APP_FOOTER_COPYRIGHT_HEIGHT, null);

const wrapper = ref<HTMLElement | null>(null);
const { top, update } = useElementBounding(wrapper);

// Content above the area (an expanding filter panel, an async latest bar) can
// move the wrapper without resizing it. Remeasure whenever the page's own size
// changes so the area keeps filling exactly up to the footer.
useResizeObserver(
    () => props.scrollContainer ?? null,
    () => update(),
);

function footerHeight(): number {
    if (props.footerCopyrightOnly && isPhone.value) {
        const visible = appFooterCopyrightHeight?.value ?? 0;
        if (visible > 0) return visible;
    }

    const measured = appFooterHeight?.value ?? 0;
    return measured > 0 ? measured : FOOTNOTE_HEIGHT;
}

function height(topValue: number) {
    const extraHeight = props.extraHeight ?? 0;
    const footer = footerHeight();
    const container = props.scrollContainer;
    const containerPadding = container
        ? Number.parseFloat(getComputedStyle(container).paddingBottom) || 0
        : 0;

    if (
        container &&
        /(auto|scroll)/.test(getComputedStyle(container).overflowY)
    ) {
        const rect = container.getBoundingClientRect();

        return availableHeight({
            viewportHeight: rect.top + container.clientHeight,
            scrollY: container.scrollTop,
            top: topValue,
            footnoteHeight: containerPadding,
            extraHeight,
            minHeight: props.minHeight,
        });
    }

    return availableHeight({
        viewportHeight: vh.value,
        scrollY: window.scrollY,
        top: topValue,
        // The page's own bottom padding sits between the area and the footer.
        footnoteHeight: footer + containerPadding,
        extraHeight,
        minHeight: props.minHeight,
    });
}

const wrapperStyle = computed(() =>
    props.autoHeight ? undefined : { height: `${height(top.value)}px` },
);
</script>

<template>
    <div ref="wrapper" :style="wrapperStyle">
        <slot></slot>
    </div>
</template>
