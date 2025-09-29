<script setup lang="ts">
import { onMounted, ref } from "vue";

const CLIENT_ID =
    "557016419724-4gvamcsq0hp8j8e0o1sjum2epq5ls446.apps.googleusercontent.com";
const SCRIPT_ID = "google-identity-services";

const buttonContainer = ref<HTMLDivElement | null>(null);

interface CredentialResponse {
    credential: string;
    select_by?: string;
    clientId?: string;
}

type GoogleAccounts = {
    id: {
        initialize: (
            config: {
                client_id: string;
                callback: (response: CredentialResponse) => void;
            }
        ) => void;
        renderButton: (element: HTMLElement, options: Record<string, unknown>) => void;
        cancel: () => void;
        prompt: () => void;
    };
};

type GoogleGlobal = {
    accounts: GoogleAccounts;
};

declare global {
    interface Window {
        google?: GoogleGlobal;
    }
}

let googleScriptPromise: Promise<void> | null = null;

function loadGoogleIdentityServices(): Promise<void> {
    if (typeof window === "undefined") {
        return Promise.resolve();
    }

    if (window.google?.accounts?.id) {
        return Promise.resolve();
    }

    if (googleScriptPromise) {
        return googleScriptPromise;
    }

    googleScriptPromise = new Promise((resolve, reject) => {
        let script = document.getElementById(SCRIPT_ID) as HTMLScriptElement | null;

        const handleLoad = () => {
            script?.setAttribute("data-loaded", "true");
            resolve();
        };

        const handleError = () =>
            reject(new Error("Failed to load Google Identity Services script"));

        if (!script) {
            script = document.createElement("script");
            script.id = SCRIPT_ID;
            script.src = "https://accounts.google.com/gsi/client";
            script.async = true;
            script.defer = true;
            script.addEventListener("load", handleLoad, { once: true });
            script.addEventListener("error", handleError, { once: true });
            document.head.appendChild(script);
            return;
        }

        if (script.getAttribute("data-loaded") === "true") {
            resolve();
            return;
        }

        script.addEventListener("load", handleLoad, { once: true });
        script.addEventListener("error", handleError, { once: true });
    });

    return googleScriptPromise;
}

function handleCredentialResponse(response: CredentialResponse) {
    console.log(`Encoded JWT ID token: ${response.credential}`);
}

function renderGoogleButton() {
    if (!buttonContainer.value) {
        return;
    }
    if (!window.google?.accounts?.id) {
        return;
    }

    buttonContainer.value.innerHTML = "";
    window.google.accounts.id.initialize({
        client_id: CLIENT_ID,
        callback: handleCredentialResponse,
    });
    window.google.accounts.id.renderButton(buttonContainer.value, {
        type: "standard",
        size: "large",
        theme: "outline",
        text: "sign_in_with",
        shape: "rectangular",
        logo_alignment: "center",
    });
}

onMounted(async () => {
    if (typeof window === "undefined") {
        return;
    }

    try {
        await loadGoogleIdentityServices();
        renderGoogleButton();
    } catch (error) {
        console.error(error);
    }
});
</script>

<template>
    <div class="google-btn-container">
        <div ref="buttonContainer"></div>
    </div>
</template>

<style scoped>
.google-btn-container {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0.5rem 0;
}
</style>
