<script setup lang="ts">
import { ref } from "vue";
import { VaButton, VaDropdown, VaIcon } from "vuestic-ui";

export interface SortOption {
    value: string;
    label: string;
    defaultDirection?: "asc" | "desc";
}

const props = defineProps<{
    options: SortOption[];
    field: string;
    direction: "asc" | "desc";
}>();

const emit = defineEmits<{
    (e: "update:field", value: string): void;
    (e: "update:direction", value: "asc" | "desc"): void;
}>();

const isOpen = ref(false);

function select(option: SortOption) {
    if (option.value === props.field) {
        emit("update:direction", props.direction === "asc" ? "desc" : "asc");
    } else {
        emit("update:field", option.value);
        emit(
            "update:direction",
            option.defaultDirection ??
                (option.value === "date" ? "desc" : "asc"),
        );
    }
    isOpen.value = false;
}
</script>

<template>
    <VaDropdown v-model="isOpen" placement="bottom-end" :offset="6">
        <template #anchor>
            <VaButton
                preset="plain"
                color="#FFFFFF"
                aria-label="排序"
                :aria-expanded="isOpen"
            >
                <VaIcon name="filter_list" size="large" class="mr-2" />
                <div class="text-lg text-center">排序</div>
            </VaButton>
        </template>

        <div
            class="min-w-44 rounded-lg border border-white/10 bg-zinc-900/95 p-1 shadow-xl backdrop-blur"
            role="menu"
            aria-label="排序方式"
        >
            <button
                v-for="option in options"
                :key="option.value"
                type="button"
                role="menuitem"
                class="flex w-full items-center justify-between gap-3 rounded-md px-3 py-2 text-left text-sm text-zinc-200 transition-colors hover:bg-white/10"
                @click="select(option)"
            >
                <span>{{ option.label }}</span>
                <VaIcon
                    v-if="option.value === field"
                    :name="
                        direction === 'asc' ? 'arrow_upward' : 'arrow_downward'
                    "
                    size="small"
                    class="text-zinc-300"
                />
            </button>
        </div>
    </VaDropdown>
</template>
