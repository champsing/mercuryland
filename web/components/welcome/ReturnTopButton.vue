<script setup lang="ts">
import { AngleUp } from "@vicons/fa";
import { VaIcon } from "vuestic-ui";

function backToTop() {
    // On the welcome page the snap-container owns the scroll, not window
    const snap = document.querySelector(".welcome-snap-container");
    if (snap) {
        snap.scrollTo({ top: 0, behavior: "smooth" });
    } else {
        window.scrollTo({ top: 0, behavior: "smooth" });
    }
}
</script>

<template>
    <div class="absolute right-6 bottom-6 z-20 anim-float">
        <!-- Outer container: 128px gives the curved text 8px breathing room
             on every side so characters at the ring edge aren't clipped -->
        <div class="relative w-[128px] h-[128px]">
            <!-- Inner visual area: centered 112px, same size as before -->
            <div
                class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-[112px] h-[112px]"
            >
                <!-- Iridescent rotating ring -->
                <div
                    class="absolute inset-0 rounded-full iridescent-ring"
                    aria-hidden="true"
                />

                <!-- Inner mask to carve out the ring shape -->
                <div
                    class="absolute inset-[6px] rounded-full bg-neutral-900/95 backdrop-blur-sm"
                    aria-hidden="true"
                />

                <!-- SVG curved text riding the ring -->
                <svg
                    viewBox="0 0 112 112"
                    class="absolute inset-0 w-full h-full z-10 pointer-events-none overflow-visible"
                    aria-hidden="true"
                >
                    <defs>
                        <path
                            id="return-curve"
                            d="M 56,3 A 53,53 0 1,1 56,109 A 53,53 0 1,1 56,3"
                        />
                    </defs>
                    <text class="curved-text">
                        <textPath href="#return-curve" startOffset="0%">
                            ↑ RETURN TO TOP&ensp;•&ensp;↑ RETURN TO TOP&ensp;•
                        </textPath>
                    </text>
                </svg>

                <!-- Center button -->
                <button
                    class="group absolute inset-[20px] rounded-full z-20 bg-gradient-to-br from-white/10 to-white/5 border border-white/15 flex items-center justify-center hover:scale-110 hover:border-cyan-400/40 hover:shadow-[0_0_25px_rgba(34,211,238,0.35)] active:scale-95 transition-all duration-300 ease-out cursor-pointer"
                    @click="backToTop()"
                    aria-label="Return to top"
                >
                    <VaIcon
                        size="36px"
                        class="text-white/75 group-hover:text-white transition-colors"
                    >
                        <angle-up />
                    </VaIcon>
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* ---- Iridescent ring: conic-gradient that continuously rotates ---- */
@keyframes spin-iridescent {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

.iridescent-ring {
    background: conic-gradient(
        from 0deg,
        #ff1493,
        #ff6600,
        #ffdd00,
        #00ff88,
        #00ccff,
        #8b5cf6,
        #ff1493
    );
    animation: spin-iridescent 3s linear infinite;
    filter: saturate(1.3);
    box-shadow:
        0 0 20px rgba(255, 255, 255, 0.12),
        0 0 40px rgba(139, 92, 246, 0.12),
        0 0 60px rgba(0, 204, 255, 0.1);
}

/* ---- Curved text: pulses between translucent and bright ---- */
@keyframes text-glow {
    0%,
    100% {
        fill: rgba(255, 255, 255, 0.5);
    }
    50% {
        fill: rgba(255, 255, 255, 0.95);
    }
}

.curved-text {
    fill: rgba(255, 255, 255, 0.7);
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.1em;
    animation: text-glow 2s ease-in-out infinite;
}
</style>
