import { reactive } from "vue";

type AuthState = {
    isAuthenticated: boolean;
    /** Whether the initial token check (auth tick) has finished. */
    isResolved: boolean;
};

const state = reactive<AuthState>({
    isAuthenticated: false,
    isResolved: false,
});

export function useAuthState(): AuthState {
    return state;
}
