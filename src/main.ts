import { createApp } from "vue";
import App from "./App.vue";
import router from "./router.ts";
import "./style.css";
import "vuestic-ui/styles/essential.css";
import "vuestic-ui/styles/typography.css";

createApp(App)
    .use(router)
    .mount("#app");
