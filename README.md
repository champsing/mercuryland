# 水星樂園 Mercury Land

> 水星人的夢幻樂園 — 惡靈 Oreki（Mercury Man）粉絲社群官方網站

[![Version](https://img.shields.io/badge/version-2.23.1-blue)](https://github.com/champsing/mercuryland)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](./LICENSE)
[![Website](https://img.shields.io/badge/website-mercuryland.pp.ua-brightgreen)](https://mercuryland.pp.ua)
[![Check](https://github.com/champsing/mercuryland/actions/workflows/check.yml/badge.svg)](https://github.com/champsing/mercuryland/actions/workflows/check.yml)
[![Publish](https://github.com/champsing/mercuryland/actions/workflows/publish.yml/badge.svg)](https://github.com/champsing/mercuryland/actions/workflows/publish.yml)

---

## 專案簡介

水星樂園（又稱「水星樂園地球分部」）是為馬來西亞實況主**惡靈 Oreki**打造的粉絲社群網站。專案把 **VOD 直播隨選**、**直播懲罰追蹤**、**水星幣虛擬經濟**、**Discord 社群機器人**與 **YouTube 直播聊天室監聽**整合在同一個服務裡。

後端是一個 **Rust** 二進位檔（edition 2024），在 tokio 執行環境中並行運行三個任務：

1. **Web API**：actix-web，監聽 `0.0.0.0:8080`，提供 23 條已註冊的 `/api` 路由。
2. **Discord 機器人**：poise / serenity，提供 10 個頂層斜線指令（含子指令共 20 個）。
3. **YouTube 監聽器**：每 60 秒檢查直播狀態，透過 YouTube Data API v3 讀取聊天室並即時計幣。

前端是 **Vue 3 + TypeScript** 製作的暗色系 SPA，使用 Vite 7、vue-router 4、Vuestic UI 與 Tailwind CSS。頁面涵蓋歡迎頁、水星伺服器、直播隨選、直播懲罰、幸運轉盤、水星排行、管理設定、聯絡我們與法務頁面。

### 功能亮點

- **歡迎頁** — 五張全螢幕投影片；桌面版以滾輪逐頁切換，手機版改為正常堆疊捲動。
- **水星伺服器** — 「加入伺服」、「法規與資料」、「地圖檔」三個分頁，集中社群規範與存檔。
- **直播隨選** — VOD 資料庫，支援日期／TAG 篩選、卡片／表格檢視、排序與直播時數計算。
- **直播懲罰** — 以「列表／統計／圖例」分頁呈現；統計頁可依狀態篩選清單、複製名稱並查看圖表。
- **幸運轉盤** — 以 `名稱x權重` 設定權重，抽出下一場懲罰；管理員可廣播結果至 Discord。
- **水星幣** — 由 YouTube 聊天室即時計幣，支援排行榜、加倍卡與加班台時數購買。
- **水星排行** — 水星幣前 50 名，前三名以頒獎台呈現。
- **管理設定** — 資料庫備份、Discord 頻道設定與匿名頻道紀錄。

---

## 目錄

- [專案架構](#專案架構)
- [技術棧](#技術棧)
- [功能總覽](#功能總覽)
- [前端頁面](#前端頁面)
- [API 文件](#api-文件)
- [Discord 指令](#discord-指令)
- [資料庫結構](#資料庫結構)
- [水星幣經濟](#水星幣經濟)
- [身分驗證](#身分驗證)
- [本地開發](#本地開發)
- [測試](#測試)
- [CI/CD](#cicd)
- [部署](#部署)
- [專案結構](#專案結構)
- [第三方整合](#第三方整合)
- [開發者備註](#開發者備註)
- [常見問題](#常見問題)
- [貢獻指南](#貢獻指南)
- [授權](#授權)

---

## 專案架構

```
+--------------------------------------------------------------+
|                        mercury_land                          |
|                     data/sqlite.db                           |
|              SQLite (WAL) + r2d2 連線池（上限 10）             |
+--------------------------------------------------------------+
        |                  |                    |
        v                  v                    v
+---------------+  +----------------+  +---------------------+
|  webpage::run |  |  discord::run  |  |    youtube::run     |
|  actix-web 4  |  | poise/serenity |  | YouTube Data API v3 |
|  HTTP :8080   |  |  10 個頂層指令  |  | 直播聊天室監聽 (60s)  |
|  23 條 API    |  |  20 個含子指令  |  | 水星幣即時計幣       |
+---------------+  +----------------+  +---------------------+
```

- 三個任務由 `src/main.rs` 以 `tokio::spawn` 啟動；任一任務失敗時會記錄錯誤，並在 60 秒後重啟。
- 收到 Ctrl+C 時，Web 伺服器會停止接受新請求並優雅關機。
- `webpage::run` 另外啟動匿名頻道快取更新器，每 30 分鐘向 Discord 更新一次作者暱稱與頭像。
- 三個任務共用同一個 SQLite 連線池與 `data/` 執行期資料。

---

## 技術棧

### 前端（`web/`）

| 類別     | 技術                                                                                          |
| -------- | --------------------------------------------------------------------------------------------- |
| 框架     | Vue 3.4（`<script setup>` SFC）、TypeScript                                                   |
| 建置     | Vite 7、vue-tsc                                                                               |
| 路由     | vue-router 4（history 模式）                                                                  |
| UI       | Vuestic UI 1.10（暗色主題）、Tailwind CSS 3、SCSS                                             |
| HTTP     | axios                                                                                         |
| 圖表     | echarts 6、vue-echarts                                                                        |
| 工具     | @vueuse/core、@vueuse/components                                                              |
| 圖示     | @vicons（Fluent / Ionicons / Tabler / Font Awesome）                                          |
| 轉盤     | vue-wheel-spinner                                                                             |
| 字型     | Google Fonts：Source Sans Pro、Playfair Display、Material Icons                               |
| 測試     | Vitest 4 + jsdom；測試檔與被測 composable 放在同一資料夾                                      |
| 格式化   | Prettier（tabWidth 4）                                                                        |
| 路徑別名 | `@` → `web/`、`@assets` → `web/assets/`、`@styles` → `styles/`、`@components`、`@composables` |

### 後端（Rust，edition 2024）

| 類別     | 技術                                                                        |
| -------- | --------------------------------------------------------------------------- |
| Web      | actix-web 4.9（actix-cors、actix-files、actix-multipart）                   |
| 資料庫   | rusqlite 0.32（bundled SQLite）、r2d2 + r2d2_sqlite、sea-query 0.32         |
| Discord  | poise 0.6.1、serenity 0.12.4                                                |
| YouTube  | google-youtube3 6.0、yup_oauth2（Device Flow）、reqwest                     |
| 認證     | JWT（HMAC-SHA256）、hmac、sha2                                              |
| 執行環境 | tokio 1.44（multi-thread）                                                  |
| 其他     | chrono、uuid（v4/v5/v7）、regex、once_cell、itertools、tempfile、env_logger |

### 資料庫

| 項目     | 內容                                                          |
| -------- | ------------------------------------------------------------- |
| 引擎     | SQLite，單一檔案 `data/sqlite.db`                             |
| 連線設定 | `journal_mode=WAL`、`synchronous=NORMAL`、`busy_timeout=5000` |
| 連線池   | r2d2，最大 10 條連線；另有未關閉連線的除錯追蹤                |
| 版本管理 | `PRAGMA user_version`，目前 **v12**；依序套用 12 個遷移       |
| 約束     | 所有資料表皆**沒有 FOREIGN KEY**；跨表關聯由程式邏輯維護      |

---

## 功能總覽

### 歡迎頁

- 五張全螢幕投影片，內容由 `web/components/welcome/Slide1.vue` 到 `Slide5.vue` 組成。
- 桌面版以 `welcome-snap-container` 攔截滾輪，一次切換一頁；手機版改用一般垂直捲動。
- `server_info.json` 的 `online` 欄位決定顯示五張投影片或「伺服器關閉中」的 Slide 1。

### 水星伺服器

`/server` 以三個分頁呈現：

| 分頁       | 內容                                                               |
| ---------- | ------------------------------------------------------------------ |
| 加入伺服   | 伺服器狀態、IP、種子碼、版本、Discord 邀請與白名單申請步驟         |
| 法規與資料 | 由 `law_document.json` 渲染的水星法規；`NewLaw` 可在有新版本時啟用 |
| 地圖檔     | 由 `map_archives.json` 產生的地圖存檔列表，點擊後另開新分頁        |

### 直播隨選（VOD）

- 以日期區間與 TAG 篩選；TAG 可切換「符合一項／符合全部」。
- 卡片／表格兩種檢視（偏好記在 `localStorage`）；可依日期或名稱排序。
- 每筆 VOD 可開啟對應的 YouTube 直播連結；管理員可新增、編輯、刪除。
- 新增時可輸入 YouTube 影片 ID，由 `/api/video/metadata` 抓取標題、日期與時長。
- 底部「剩餘時間」摘要與「計算明細」以 `schedule.json` 與目前篩選的 VOD 資料計算計畫、直播、懲罰與課金時數。

### 直播懲罰（Penalty）

`/penalty` 以三個分頁呈現：

| 分頁 | 內容                                                                                                                                                                                                   |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 列表 | 最新懲罰細長狀態列；日期、搜尋與完成狀態篩選；卡片／表格檢視；日期／名稱／狀態排序；點擊列開啟詳情與時間軸。管理員可新增、編輯。                                                                       |
| 統計 | 現存／完成兩組統計；狀態數量方塊可切換為該狀態的篩選器（使用狀態色）；清單以唯讀文字框呈現，支援 Ctrl+A 與「複製全部／複製未完成…」動態按鈕；另有每日數量堆疊圖與狀態圓餅圖，時間範圍可選 1 天至全部。 |
| 圖例 | 詳細語法標籤與五種完成狀態的說明，以及「無法完成的懲罰與轉換條件」註腳。                                                                                                                               |

懲罰狀態：

| 狀態碼 | 狀態名稱 | 說明               |
| ------ | -------- | ------------------ |
| 0      | 未生效   | 預備懲罰，尚未生效 |
| 1      | 未完成   | 尚未開始嘗試       |
| 2      | 進行中   | 已開始、有進度     |
| 3      | 勉強過   | 以最低標準通過     |
| 4      | 已完成   | 所有條件皆已完成   |

- 詳細內容以 HTML 儲存，支援圖片上傳與嵌入內容；編輯器包含圖片、Steam、YouTube 與自訂語法工具。
- 歷史紀錄以 JSON 陣列保存，詳情頁以時間軸呈現。

### 幸運轉盤（Wheel）

- 以文字框輸入選項，格式為 `名稱x權重`（例如 `喝水x3`）；未指定權重時視為 1。
- 轉盤使用加權隨機抽出結果；可鎖定左側待抽區、切換是否清除右側結果。
- 需要管理員登入才能將結果廣播到 Discord。
- `POST /api/wheel/submit` 會依日期（UTC+8）建立懲罰資料，並將結果送到 `ChannelPenalty` 設定的頻道。
- 轉盤狀態保存在瀏覽器端，伺服器不保存輪盤設定。

### 水星幣（Mercury Coin）

- YouTube 監聽器每 60 秒確認直播，讀取聊天室文字訊息並即時計幣。
- 計幣規則、加倍卡與加班台時數詳見[水星幣經濟](#水星幣經濟)。
- 可透過 `/coin` 查詢餘額、`/link`／`/unlink` 連結或解除連結 YouTube 頻道。
- 排行榜頁面取水星幣前 50 名。

### 水星排行（Leaderboard）

- 依水星幣餘額由高到低排序，前三名以頒獎台呈現，其餘列為表格。
- 可手動重新整理；頁面顯示按讚暱稱、YouTube 頻道、水星幣與最後更新時間。

### 管理設定（`/setting`）

僅在管理員登入後可操作，頁面包含三張卡片：

| 卡片       | 功能                                                     |
| ---------- | -------------------------------------------------------- |
| 資料庫備份 | 透過 SQLite Backup API 產生備份並下載                    |
| 頻道設定   | 讀寫 `config` 表中的五個 Discord／YouTube 設定值         |
| 匿名頻道   | 顯示匿名頻道訊息紀錄（前端已接上 `/api/anonymous/list`） |

### 其他頁面

- **聯絡我們**：Discord 與其他聯絡方式。
- **服務條款 / 隱私權政策**：`/tos`、`/privacy`。
- **404**：所有未匹配路徑顯示 `NotFound.vue`。

---

## 前端頁面

| 路由           | 頁面        | 說明                               |
| -------------- | ----------- | ---------------------------------- |
| `/`            | Welcome     | 五張投影片的歡迎頁                 |
| `/server`      | Server      | 加入伺服／法規與資料／地圖檔       |
| `/vod`         | VOD         | 直播隨選、TAG 篩選與時數計算       |
| `/penalty`     | Penalty     | 列表／統計／圖例                   |
| `/contact`     | Contact     | 聯絡我們                           |
| `/wheel`       | Wheel       | 加權幸運轉盤                       |
| `/tos`         | ToS         | 服務條款                           |
| `/privacy`     | Privacy     | 隱私權政策                         |
| `/leaderboard` | Leaderboard | 水星幣排行與前三名頒獎台           |
| `/setting`     | Setting     | 管理員設定（導覽列僅在登入後顯示） |
| 其他路徑       | NotFound    | `/:pathMatch(.*)*` 的 404 頁面     |

> 路由定義在 `web/router.ts`，共 10 個頁面路由加 1 個 catch-all。

---

## API 文件

所有路由以 `/api` 開頭，由 actix-web 提供，只接受 GET 與 POST。CORS 允許以下來源：

- `http://localhost:5173`
- `https://mercuryland.pp.ua`
- `https://www.mercuryland.pp.ua`

管理員路由需要驗證 JWT。驗證方式依 handler 而異：`/api/setting/*` 使用 `Authorization: Bearer <token>`；VOD、懲罰、轉盤與圖片上傳則讀取請求 body／multipart 中的 `token` 欄位。前端 axios 攔截器會在非白名單請求自動加上 Bearer；白名單包含 penalty list、penalty detail、video list、leaderboard、auth login 與 ping。

### 認證

| 方法 | 路徑              | 說明                              | 權限 |
| ---- | ----------------- | --------------------------------- | ---- |
| GET  | `/api/ping`       | 健康檢查                          | 公開 |
| POST | `/api/auth/login` | 以 Discord `/auth` 驗證碼換取 JWT | 公開 |
| POST | `/api/auth/tick`  | JWT 續期（新的 1 小時有效期）     | 公開 |

### 轉盤

| 方法 | 路徑                | 說明                                      | 權限   |
| ---- | ------------------- | ----------------------------------------- | ------ |
| POST | `/api/wheel/submit` | 建立懲罰並廣播至 Discord（token 在 body） | 管理員 |

### VOD

| 方法 | 路徑                     | 說明                            | 權限   |
| ---- | ------------------------ | ------------------------------- | ------ |
| GET  | `/api/video/list`        | 列出全部 VOD                    | 公開   |
| POST | `/api/video/upload-json` | 以 multipart 上傳 JSON 批次匯入 | 管理員 |
| POST | `/api/video/insert`      | 新增單筆 VOD                    | 管理員 |
| POST | `/api/video/delete`      | 刪除 VOD                        | 管理員 |
| POST | `/api/video/update`      | 更新 VOD                        | 管理員 |
| POST | `/api/video/metadata`    | 依 YouTube 影片 ID 抓取後設資料 | 公開   |

### 懲罰

| 方法 | 路徑                          | 說明               | 權限   |
| ---- | ----------------------------- | ------------------ | ------ |
| GET  | `/api/penalty/list`           | 懲罰列表           | 公開   |
| GET  | `/api/penalty/detail/{id}`    | 單筆懲罰詳情       | 公開   |
| POST | `/api/penalty/insert`         | 新增懲罰           | 管理員 |
| POST | `/api/penalty/delete`         | 刪除懲罰           | 管理員 |
| POST | `/api/penalty/update`         | 更新懲罰           | 管理員 |
| POST | `/api/penalty/detail/update`  | 更新懲罰 HTML 詳情 | 管理員 |
| POST | `/api/penalty/history/update` | 更新懲罰歷史紀錄   | 管理員 |

### 排行榜與設定

| 方法 | 路徑                  | 說明                      | 權限   |
| ---- | --------------------- | ------------------------- | ------ |
| GET  | `/api/leaderboard`    | 水星幣排行榜              | 公開   |
| GET  | `/api/setting/backup` | 下載 SQLite 備份          | 管理員 |
| GET  | `/api/setting/config` | 讀取設定（需 `?id=0..4`） | 管理員 |
| POST | `/api/setting/config` | 更新設定                  | 管理員 |

### 圖片

| 方法 | 路徑                    | 說明                   | 權限   |
| ---- | ----------------------- | ---------------------- | ------ |
| POST | `/api/image/upload`     | 以 multipart 上傳圖片  | 管理員 |
| GET  | `/api/image/get/{name}` | 依 UUIDv5 名稱取得圖片 | 公開   |

> 已註冊路由共 **23 條**。管理員判定方式：後端解碼 JWT 取得 Discord 使用者 ID，與 `data/config.json` 的 `discord.admin` 清單比對。

---

## Discord 指令

機器人以 poise / serenity 實作，使用全域斜線指令，指令描述與回應皆為繁體中文。共有 10 個頂層指令、20 個含子指令。

| 指令                 | 說明                                                                    |
| -------------------- | ----------------------------------------------------------------------- |
| `/auth`              | （管理）產生 8 碼英數驗證碼，30 分鐘內有效、單次使用                    |
| `/coin`              | 查詢自己或指定 YouTube 頻道的水星幣餘額                                 |
| `/give`              | （管理）給予或扣除指定使用者的水星幣                                    |
| `/link`              | 連結 Discord 與 YouTube 頻道；同一 Discord 帳號僅能連結一個頻道         |
| `/unlink`            | 解除 Discord 與 YouTube 頻道連結                                        |
| `/help`              | 顯示指令教學                                                            |
| `/refund close`      | （管理）關閉退款討論串                                                  |
| `/refund reopen`     | （管理）重新開啟退款討論串                                              |
| `/purchase booster`  | 購買懲罰加倍卡（等級 2–9 + 懲罰內容），公告於水星幣頻道                 |
| `/purchase overtime` | 購買加班台時數（小時 + 留言），每小時 1000 水星幣                       |
| `/vote nominate`     | 提名投票選項                                                            |
| `/vote revoke`       | 撤回提名                                                                |
| `/vote deadline`     | 設定投票截止時間                                                        |
| `/vote conclude`     | 結算投票結果                                                            |
| `/vote clear`        | 清除投票資料                                                            |
| `/anonymous create`  | （管理）在目前頻道建立「匿名發言」按鈕與 Modal；送出後以匿名 Embed 發佈 |

> `/link` 有 2 小時的使用冷卻。YouTube 聊天室也支援文字指令 `/purchase booster <等級> <內容>`，由監聽器直接處理。

---

## 資料庫結構

Schema 由 `src/database/migration/` 中的 **12 個已套用遷移**（v1 → v12）管理；啟動時比對 `PRAGMA user_version`，自動執行尚未套用的版本。所有資料表皆無 FOREIGN KEY。

### user（使用者與水星幣）

| 欄位         | 型別     | 限制                      | 說明                          |
| ------------ | -------- | ------------------------- | ----------------------------- |
| `id`         | INTEGER  | PRIMARY KEY AUTOINCREMENT | 主鍵                          |
| `youtube`    | TEXT     | UNIQUE, NOT NULL          | YouTube 頻道 ID（主要查詢鍵） |
| `discord`    | BIGINT   | UNIQUE, nullable          | Discord 使用者 ID             |
| `display`    | TEXT     | NOT NULL                  | 顯示名稱                      |
| `coin`       | INTEGER  | NOT NULL                  | 水星幣餘額                    |
| `updated_at` | DATETIME | NOT NULL                  | 最後更新時間                  |

### video（VOD 檔案庫）

| 欄位       | 型別    | 限制                   | 說明               |
| ---------- | ------- | ---------------------- | ------------------ |
| `id`       | INTEGER | PRIMARY KEY            | 主鍵               |
| `date`     | DATE    | NOT NULL               | 直播日期           |
| `link`     | TEXT    | NOT NULL, UNIQUE INDEX | YouTube 影片 ID    |
| `title`    | TEXT    | NOT NULL               | 影片標題           |
| `tags`     | TEXT    | NOT NULL               | 標籤 JSON 陣列     |
| `duration` | TEXT    | NOT NULL               | 時長（`HH:MM:SS`） |

### penalty（懲罰追蹤）

| 欄位      | 型別    | 限制        | 說明                                    |
| --------- | ------- | ----------- | --------------------------------------- |
| `id`      | INTEGER | PRIMARY KEY | 主鍵                                    |
| `date`    | DATE    | NOT NULL    | 排定日期                                |
| `name`    | TEXT    | NOT NULL    | 懲罰名稱                                |
| `detail`  | TEXT    | NOT NULL    | 細節內容（HTML）                        |
| `state`   | INTEGER | NOT NULL    | 狀態碼 0–4（見上方狀態表）              |
| `history` | TEXT    | NOT NULL    | 歷史紀錄 JSON（每項為 `[state, date]`） |

### image（上傳圖片）

| 欄位   | 型別    | 限制             | 說明                          |
| ------ | ------- | ---------------- | ----------------------------- |
| `id`   | INTEGER | PRIMARY KEY      | 主鍵                          |
| `name` | TEXT    | NOT NULL, UNIQUE | UUIDv5 檔名（由內容雜湊產生） |
| `data` | BLOB    | NOT NULL, UNIQUE | 原始圖片位元組                |
| `mime` | TEXT    | NOT NULL         | MIME 型別（例如 `image/png`） |

### config（系統設定鍵值）

| 欄位   | 型別    | 限制        | 說明           |
| ------ | ------- | ----------- | -------------- |
| `id`   | INTEGER | PRIMARY KEY | 設定鍵（數值） |
| `text` | TEXT    | NOT NULL    | 設定值         |

| 鍵值 | 名稱               | 用途                       |
| ---- | ------------------ | -------------------------- |
| 0    | `ChannelPenalty`   | 懲罰公告 Discord 頻道 ID   |
| 1    | `ChannelCoin`      | 水星幣公告 Discord 頻道 ID |
| 2    | `ChannelVote`      | 投票 Discord 頻道 ID       |
| 3    | `MessageVote`      | 投票訊息 ID                |
| 4    | `YoutubeChannelId` | 監聽的 YouTube 頻道 ID     |

### anonymous（匿名頻道）

| 欄位         | 型別     | 限制        | 說明                     |
| ------------ | -------- | ----------- | ------------------------ |
| `id`         | INTEGER  | PRIMARY KEY | 主鍵                     |
| `author`     | BIGINT   | NOT NULL    | 投稿者 Discord 使用者 ID |
| `updated_at` | DATETIME | NOT NULL    | 投稿時間                 |

> 原始匿名內容在 v12 已從資料庫移除；實際內容保存在 Discord 的匿名 Embed，資料庫只保留作者與時間中繼資料。

---

## 水星幣經濟

水星幣由 YouTube 直播聊天室的文字訊息即時累積。只有 `textMessageEvent` 會計幣：

| 規則             | 一般觀眾 | 頻道會員（贊助者） |
| ---------------- | -------- | ------------------ |
| 每則聊天室訊息   | 1 幣     | 2 幣               |
| 每日首則訊息獎勵 | 10 幣    | 20 幣              |
| 每日計幣上限     | 50 幣    | 100 幣             |

- 每位觀眾每 30 秒最多計入一則訊息。
- 每日額度在跨日後重置。

### 懲罰加倍卡（Booster）

加倍卡是**轉盤懲罰的加倍憑證**，不是聊天室計幣倍率。購買後機器人會在 `ChannelCoin` 頻道公告「懲罰加倍」，由管理員在轉盤上依公告內容調整 `名稱x權重`。

| 等級 | 成本（水星幣） |
| ---- | -------------- |
| 2    | 50             |
| 3    | 100            |
| 4    | 200            |
| 5    | 400            |
| 6    | 800            |
| 7    | 1,600          |
| 8    | 3,200          |
| 9    | 6,400          |

- 購買後 72 小時內可依 Discord 退款流程申請退款。

### 加班台時數

- 每小時 **1000 水星幣**（未滿一小時無條件進位）。
- `Discord /purchase overtime <hours> <content>` 會扣除水星幣並在 `ChannelCoin` 公告時數卡。
- 購買後 72 小時內可依退款流程申請退款。

> 購買管道：Discord 的 `/purchase booster` 與 `/purchase overtime`；YouTube 聊天室另外支援文字指令 `/purchase booster <等級> <內容>`。

---

## 身分驗證

本專案使用輕量自訂認證，沒有第三方 OAuth：

1. 管理員在 Discord 執行 `/auth`，機器人產生 **8 碼大寫英數驗證碼**（30 分鐘有效、單次使用）。
2. 前端在登入視窗輸入驗證碼，並附上從 `api.ipify.org` 取得的客戶端 IP，呼叫 `POST /api/auth/login`。
3. 後端驗證成功後，以 **HMAC-SHA256** 簽發有效期 1 小時的 JWT。
4. 前端將 token 存入 `localStorage`；`Login.vue` 每 10 分鐘呼叫 `POST /api/auth/tick` 續期。
5. axios 攔截器會替非白名單請求加上 `Authorization: Bearer <token>`。

> JWT 簽署金鑰在每次伺服器啟動時隨機產生。伺服器重啟後所有既有 session 立即失效，管理員必須重新登入。這是刻意的設計，避免永久管理員 session。

管理員身分由 `data/config.json` 的 `discord.admin` 陣列決定；後端解碼 JWT 後比對 Discord 使用者 ID。

---

## 本地開發

### 前置需求

- **Rust**：edition 2024，stable toolchain
- **Node.js** 20+ 與 npm
- SQLite 由 `rusqlite` 的 bundled feature 提供，無需另外安裝

### 環境變數

在專案根目錄建立 `.env`（已列入 `.gitignore`）：

| 變數            | 說明                                              |
| --------------- | ------------------------------------------------- |
| `DISCORD_TOKEN` | Discord Bot Token                                 |
| `YOUTUBE_TOKEN` | YouTube OAuth 用戶端密鑰 JSON（Device Flow 使用） |

### 設定檔

`data/config.json`：管理員 Discord 使用者 ID 清單。

```json
{
    "discord": {
        "admin": ["你的Discord使用者ID"]
    }
}
```

其他執行期檔案：

- `data/sqlite.db`：SQLite 資料庫（WAL 模式）
- `data/youtube.secret`：Device Flow 取得的 OAuth token（首次啟動後產生）

> 首次啟動 YouTube 監聽器時，若 `data/youtube.secret` 不存在，機器人會私訊第一位管理員 Device Flow 驗證碼，完成授權後才會開始監聽。

### 啟動

終端機一（後端）：

```sh
cargo run
# 預設監聽 0.0.0.0:8080
```

終端機二（前端）：

```sh
npm install
npm run dev
# Vite dev server：http://localhost:5173
```

開發時前端 API 會指向 `http://127.0.0.1:8080`（`web/composables/utils.ts` 的 `BASE_URL`）；CORS 已允許 `localhost:5173`。

### 建置與格式化

```sh
# 前端
npm run build          # 輸出至 dist/
npm run preview        # 預覽建置產物

# 後端
cargo build --release  # 輸出至 target/release/mercury_land

# 格式化
cargo fmt
npm run format         # Prettier（tabWidth 4）
```

---

## 測試

### 後端

- 使用各模組內的 `#[cfg(test)]` 單元測試。
- 執行 `cargo test`。
- 測試涵蓋資料庫遷移（從空白資料庫遷移到 v12、跳過已是最新版本、拒絕未來版本）、水星幣規則、JWT 驗證、設定存取、YouTube 直播 ID 解析等。

### 前端

- 使用 **Vitest 4 + jsdom**；測試檔位於 `web/composables/tests/`，與 composable 放在同一專案內。
- `web/test/setup.ts` 會 mock `vuestic-ui` 的 `useToast`，避免 composable 測試需要完整的 Vue 插件環境。

```sh
npm test           # 單次執行
npm run test:watch # 監看模式
npx vue-tsc --noEmit
npm run format:check
```

目前共有 **9 個測試檔、94 個測試**。

> `check.yml` 目前執行 Rust 測試、格式檢查、前端型別檢查與建置；**尚未**執行 `npm test`。送出 PR 前請在本機手動跑一次。

---

## CI/CD

`.github/workflows/` 包含三個 workflow。

### check.yml（Pull Request）

依變更路徑啟動對應工作：

| 工作    | 內容                                                                      |
| ------- | ------------------------------------------------------------------------- |
| Rust    | `cargo fmt --all -- --check`、`cargo test`                                |
| Docker  | 以 PR 的 secrets 作為 build args 建置 Docker 映像                         |
| Vue.js  | `npm ci`、`npm run format:check`、`npx vue-tsc --noEmit`、`npm run build` |
| Version | 比對 `Cargo.toml` 與 `package.json` 的版本是否一致                        |

路徑過濾：Rust 變更觸發 Rust 工作；`Dockerfile`／compose 變更觸發 Docker；`web/**`、`public/**`、前端設定檔變更觸發 Vue.js；版本檔變更觸發 Version。

### publish.yml（前端部署）

推送至 `main` 時：

1. `npm ci`
2. `npx vue-tsc --noEmit`
3. `npm run build`
4. `wrangler pages deploy dist --project-name=mercuryland` 部署至 **Cloudflare Pages**

僅在非 fork 的儲存庫執行。

### build.yml（後端部署）

推送至 `main` 且 `Cargo.toml` 的版本行有變更時：

1. Docker 多階段建置（`rust:alpine` → `scratch`）
2. `docker save` 匯出映像檔
3. 透過 **cloudflared SSH 隧道**上傳至遠端伺服器
4. 遠端執行 `docker load` 並重啟 compose 服務

---

## 部署

### 前端

- `https://mercuryland.pp.ua`，由 `publish.yml` 部署至 Cloudflare Pages。
- `BASE_URL` 在 production 指向 `https://api.mercuryland.pp.ua`。

### 後端

- Dockerfile 使用 `rust:alpine` 編譯靜態 musl 二進位，再放入 `scratch` 映像。
- 容器工作目錄為 `/mercury`，對外暴露 `8080`。
- 建置時以 `--build-arg` 注入 `DISCORD_TOKEN` 與 `YOUTUBE_TOKEN`。
- `data/`（SQLite、`config.json`、`youtube.secret`）需在部署端持久化掛載。
- `api.mercuryland.pp.ua` 經 Cloudflare Tunnel 反向代理至容器。
- 部署由 `build.yml` 自動化（cloudflared SSH + `docker load` + 遠端重啟腳本）。

---

## 專案結構

```
mercuryland/
├── web/                              # Vue 3 前端
│   ├── main.ts                       # 應用程式進入點（Vuestic、router、全域樣式）
│   ├── router.ts                     # 10 個頁面路由 + 1 個 404 catch-all
│   ├── App.vue                       # 導覽列、登入、footer 與 router-view
│   ├── Leaderboard.vue               # 水星排行（前 50 名、前三名頒獎台）
│   ├── NotFound.vue                  # 404 頁面
│   ├── SortMenu.vue                  # 共用排序選單
│   ├── ViewModeToggle.vue            # 卡片／表格檢視切換
│   ├── ViewportHeight.vue            # 讓列表貼合可視高度的輔助元件
│   ├── components/
│   │   ├── welcome/                  # 歡迎頁五張投影片與動畫
│   │   ├── server/                   # Server／Join／Law／NewLaw／Archive
│   │   ├── vod/                      # VOD 頁、列表、新增／編輯、時數計算
│   │   ├── penalty/                  # 懲罰頁、列表、詳情、時間軸、編輯器
│   │   │   └── tabs/                 # 列表最新懲罰／統計／圖表／圖例分頁
│   │   ├── wheel/                    # 幸運轉盤與 Spinner
│   │   ├── setting/                  # 備份、頻道設定、匿名頻道紀錄
│   │   ├── login/                    # 管理員登入／登出
│   │   ├── contact/                  # 聯絡我們
│   │   └── law/                      # 服務條款、隱私權政策
│   ├── composables/                  # axios、authState、penalty、vod、storage、viewMode 等
│   │   └── tests/                    # composable 的 Vitest 測試
│   ├── assets/data/                  # 靜態 JSON（法規、地圖、排程、伺服器資訊等）
│   │   └── penalty_history/          # 懲罰歷史資料（history／latest／calc_history）
│   └── test/setup.ts                 # Vitest setup（mock useToast）
├── src/                              # Rust 後端
│   ├── main.rs                       # 三個 tokio 任務的入口
│   ├── lib.rs                        # 模組宣告
│   ├── config.rs                     # .env、data/config.json、驗證碼狀態
│   ├── error.rs                      # ServerError
│   ├── webpage/                      # actix-web API
│   │   ├── mod.rs                    # 23 條路由註冊、CORS、匿名快取啟動
│   │   ├── auth/                     # login、tick、JWT 驗證
│   │   ├── wheel/                    # submit（update.rs 未編譯）
│   │   ├── video/                    # VOD CRUD、metadata、批次上傳
│   │   ├── penalty/                  # 懲罰 CRUD、詳情、歷史
│   │   ├── leaderboard/              # 排行榜
│   │   ├── setting/                  # 設定讀寫、資料庫備份
│   │   ├── image/                    # 圖片上傳與讀取
│   │   ├── anonymous/                # 匿名頻道快取與列表 handler
│   │   └── ping.rs                   # 健康檢查
│   ├── discord/                      # poise/serenity 指令與事件處理
│   │   └── anonymous.rs              # 匿名按鈕／Modal 事件處理
│   ├── youtube/                      # 直播偵測與聊天室監聽
│   ├── coin/                         # 水星幣規則與聊天室指令
│   └── database/                     # SQLite 存取層
│       └── migration/                # v1 → v12 的 12 個已套用遷移
├── styles/                           # style.css、va-modal-glass.css、welcome-animations.css
├── public/                           # 圖示、歡迎頁圖片、音效、Google 驗證與 .well-known
├── data/                             # 執行期資料（sqlite.db、config.json、youtube.secret）
├── scripts/                          # 懲罰歷史整理腳本（calc_history.py、read_history.py）
├── insert_latest.py                  # 一次性資料匯入輔助腳本
├── .github/workflows/                # check.yml、publish.yml、build.yml
├── Dockerfile
├── Cargo.toml / Cargo.lock
├── package.json / package-lock.json
├── vite.config.ts / vitest.config.ts
├── tailwind.config.js / postcss.config.js
└── tsconfig.json / tsconfig.node.json
```

---

## 第三方整合

| 服務                    | 用途                                                         |
| ----------------------- | ------------------------------------------------------------ |
| **Discord**             | 機器人、斜線指令、驗證碼登入、公告、匿名頻道與退款流程       |
| **YouTube Data API v3** | 直播偵測、聊天室訊息讀取（60 秒輪詢）、OAuth 2.0 Device Flow |
| **YouTube 直播頁面**    | 由 `/channel/{id}/live` 的 HTML 解析直播 video ID            |
| **Cloudflare Pages**    | 前端靜態託管與自動部署                                       |
| **Cloudflare Tunnel**   | 後端 SSH 部署隧道與 `api.mercuryland.pp.ua` 反向代理         |
| **Google Fonts**        | Source Sans Pro、Playfair Display、Material Icons            |
| **api.ipify.org**       | 登入時取得客戶端 IP，供後端記錄登入來源                      |

---

## 開發者備註

以下是目前程式碼中已知、但尚未影響主要流程的落差：

- `src/webpage/anonymous/list.rs` 定義了 `GET /api/anonymous/list`，但 `src/webpage/mod.rs` 目前**沒有註冊**這個 service；設定頁的「匿名頻道」卡片會因此載入失敗。
- v1 曾建立 `wheel` 資料表；目前的轉盤流程只使用 `/api/wheel/submit`。`src/webpage/wheel/update.rs` 未被 `wheel/mod.rs` 宣告編譯，`011_drop_wheel_table.sql` 也未列入遷移鏈。
- `check.yml` 尚未執行 Vitest；前端測試需在本機以 `npm test` 執行。

---

## 常見問題

### 水星幣怎麼取得？

在 YouTube 直播聊天室發送文字訊息即可自動獲得。一般觀眾每則 1 幣、每日首則 10 幣、每日上限 50 幣；頻道會員加倍。詳見[水星幣經濟](#水星幣經濟)。

### 為什麼伺服器重啟後需要重新登入？

JWT 簽署金鑰在每次啟動時隨機產生，重啟後所有既有 token 立即失效。這是刻意的安全設計，確保沒有永久管理員 session。

### 資料庫檔案在哪裡？

`data/sqlite.db`（WAL 模式，附帶 `-wal` 與 `-shm`）。可從管理設定下載備份，或直接以 `sqlite3` 查詢。

### 如何成為管理員？

將 Discord 使用者 ID 加入 `data/config.json` 的 `discord.admin` 陣列，重啟服務後生效；接著在 Discord 執行 `/auth` 取得登入驗證碼。

### 前端開發時 API 連不上？

確認後端已以 `cargo run` 啟動（8080 port）。開發時前端 `BASE_URL` 會指向 `http://127.0.0.1:8080`，CORS 也已允許 `localhost:5173`。

### 為什麼加水星幣的「加倍卡」沒有讓聊天室幣變多？

加倍卡是**轉盤懲罰**的加倍憑證，不是聊天室計幣倍率。聊天室計幣固定依[水星幣經濟](#水星幣經濟)的規則計算。

---

## 貢獻指南

1. Fork 專案並建立 feature branch。
2. 修改後執行格式化與測試：

    ```sh
    cargo fmt
    cargo test
    npm run format
    npm test
    npx vue-tsc --noEmit
    ```

3. 若變更涉及版本號，**同步更新** `Cargo.toml` 與 `package.json`（CI 會檢查一致性）。
4. 發起 Pull Request 至 `main`，CI 會執行 `check.yml`。

---

## 授權

本專案採用 [Apache License 2.0](./LICENSE) 釋出。

---

<p align="center">
  <a href="https://mercuryland.pp.ua">水星樂園地球分部</a>
  ·
  <a href="https://github.com/champsing/mercuryland">GitHub</a>
</p>
