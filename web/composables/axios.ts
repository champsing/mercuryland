import axios from "axios";
import { BASE_URL } from "./utils";

const api = axios.create({
    baseURL: BASE_URL,
});

// 使用 Regex 定義白名單
const whiteList = [
    /^\/api\/penalty\/list$/, // 精確匹配 list
    /^\/api\/penalty\/detail\/[a-zA-Z0-9_-]+$/, // 只匹配 detail/{id}，不允許後面還有 /update
    /^\/api\/video\/list/, // 匹配 video list 開頭的所有請求
    /^\/api\/leaderboard/, // 匹配 leaderboard 開頭的所有請求
    /^\/api\/auth\/login$/, // 精確匹配 login
];

api.interceptors.request.use((config) => {
    // 取得不含 BASE_URL 的純路徑
    const url = config.url || "";

    // 使用 test() 檢查是否符合正則表達式
    const isWhiteListed = whiteList.some((pattern) => {
        if (pattern instanceof RegExp) {
            
            return pattern.test(url);
            
        }
        return url.includes(pattern); // 保留對字串的兼容
    });

    if (isWhiteListed) {
        return config; // 如果在白名單，直接放行，不檢查 token 也不加 header
    }

    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return Promise.reject(new Error("未找到認證 Token，請重新登錄"));
    } else {
        config.headers.Authorization = `Bearer ${token}`;
    }

    return config;
});

export default api;
