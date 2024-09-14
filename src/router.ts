import { createWebHashHistory, createRouter } from "vue-router";
import Vod from "@components/vod/Vod.vue";
import Penalty from "@components/penalty/Penalty.vue";
// import Welcome from "@components/welcome/Welcome.vue";
// import Join from "@/components/join/Join.vue";
// import Publication from "@components/publication/Publication.vue";
// import GameMap from "@components/GameMap.vue";
// import Contact from "@components/contact/Contact.vue";

const routes = [
    // { path: "/", component: Welcome },
    // { path: "/join", component: Join },
    // { path: "/publication", component: Publication },
    // { path: "/contact", component: Contact },
    // { path: "/map", component: GameMap },
    { path: "/", component: Vod },
    { path: "/penalty", component: Penalty },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

export default router;
