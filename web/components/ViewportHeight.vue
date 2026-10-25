<script setup lang="ts">
import { APP_FOOTER_HEIGHT } from "@/composables/appFooter";
import { availableHeight, FOOTNOTE_HEIGHT } from "@/composables/viewportHeight";
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";
import { inject } from "vue";

const props = defineProps<{
    /** Height added back so collapsible panels above do not shrink the area. */
    extraHeight?: number;
    /** Page scroll container; when it actually scrolls, measure against it. */
    scrollContainer?: HTMLElement | null;
    /** Smallest usable area height; defaults to the shared minimum. */
    minHeight?: number;
}>();

const vh = useWindowSize().height;
const appFooterHeight = inject(APP_FOOTER_HEIGHT, null);

function footerHeight(): number {
    const measured = appFooterHeight?.value ?? 0;
    return measured > 0 ? measured : FOOTNOTE_HEIGHT;
}

function height(top: number) {
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
            top,
            footnoteHeight: containerPadding,
            extraHeight,
            minHeight: props.minHeight,
        });
    }

    return availableHeight({
        viewportHeight: vh.value,
        scrollY: window.scrollY,
        top,
        // The page's own bottom padding sits between the area and the footer.
        footnoteHeight: footer + containerPadding,
        extraHeight,
        minHeight: props.minHeight,
    });
}
</script>

<template>
    <UseElementBounding v-slot="{ top }">
        <div :style="{ height: `${height(top)}px` }">
            <slot></slot>
        </div>
    </UseElementBounding>
</template>
