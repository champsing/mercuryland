import { createWebHistory, createRouter } from "vue-router";
import Vod from "@components/vod/Vod.vue";
import Penalty from "@components/penalty/Penalty.vue";
// import Join from "@/components/join/Join.vue";
// import Publication from "@components/publication/Publication.vue";
// import GameMap from "@components/GameMap.vue";
import Contact from "@components/contact/Contact.vue";
import ToS from "./components/law/ToS.vue";
import Wheel from "./components/wheel/Wheel.vue";
import Privacy from "./components/law/Privacy.vue";
import Leaderboard from "./components/Leaderboard.vue";
import Setting from "./components/setting/Setting.vue";
import Welcome2 from "./components/welcome/Welcome2.vue";

const routes = [
    { path: "/", component: Welcome2 },
    // { path: "/join", component: Join },
    // { path: "/publication", component: Publication },
    // { path: "/map", component: GameMap },
    { path: "/vod", component: Vod },
    { path: "/penalty", component: Penalty },
    { path: "/contact", component: Contact },
    { path: "/wheel", component: Wheel },
    { path: "/tos", component: ToS },
    { path: "/privacy", component: Privacy },
    { path: "/leaderboard", component: Leaderboard },
    { path: "/setting", component: Setting },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
