import { createWebHashHistory, createRouter } from "vue-router";
import Welcome from "@components/welcome/Welcome.vue";
import Vod from "@components/vod/Vod.vue";
import GameMap from "@components/GameMap.vue";
import Penalty from "@components/penalty/Penalty.vue";
import Publication from "@components/publication/Publication.vue";
import Contact from "@components/contact/Contact.vue";
import Join from "@components/Join.vue";

const routes = [
    { path: "/", component: Welcome },
    { path: "/vod", component: Vod },
    { path: "/map", component: GameMap },
    { path: "/penalty", component: Penalty },
    { path: "/publication", component: Publication },
    { path: "/contact", component: Contact },
    { path: "/join", component: Join },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

export default router;
