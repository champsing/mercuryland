import { reactive } from "vue";

type AuthState = {
    isAuthenticated: boolean;
};

const state = reactive<AuthState>({
    isAuthenticated: false,
});

export function useAuthState(): AuthState {
    return state;
}
