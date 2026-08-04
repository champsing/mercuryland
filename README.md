# 水星樂園 Mercury Land

> 水星人的夢幻樂園 — 惡靈（Mercury Man）粉絲社群官方網站

[![Version](https://img.shields.io/badge/version-2.23.1-blue)](https://github.com/champsing/mercuryland)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](./LICENSE)
[![Website](https://img.shields.io/badge/website-mercuryland.pp.ua-brightgreen)](https://mercuryland.pp.ua)
[![Check](https://github.com/champsing/mercuryland/actions/workflows/check.yml/badge.svg)](https://github.com/champsing/mercuryland/actions/workflows/check.yml)
[![Publish](https://github.com/champsing/mercuryland/actions/workflows/publish.yml/badge.svg)](https://github.com/champsing/mercuryland/actions/workflows/publish.yml)

---

## 專案簡介

水星樂園（又稱「水星樂園地球分部」）是為馬來西亞實況主**惡靈 Oreki**打造的粉絲社群網站。本專案整合了 **VOD 直播隨選資料庫**、**直播懲罰追蹤**、**水星幣虛擬經濟**、**Discord 社群機器人**與 **YouTube 直播聊天室即時串接**，讓觀眾在直播之外也能延續互動與回憶。

網站以單一 **Rust** 後端服務同時驅動三大任務：**Web API 伺服器**（actix-web，port 8080）、**Discord 機器人**（poise/serenity，斜線指令）與 **YouTube 直播聊天室監聽器**（YouTube Data API v3，60 秒輪詢）。前端則以 **Vue 3** 打造暗色系沉浸式體驗，從歡迎頁的五張投影片展開旅程，一路通往互動地圖、隨選影片、懲罰紀錄、幸運轉盤與水星排行。

### 功能亮點

- **直播隨選** — 完整 VOD 資料庫，附時間換算工具
- **直播懲罰** — 五階段狀態追蹤（未生效 → 已完成），含時間軸與歷史紀錄
- **幸運轉盤** — 轉盤決定下一場懲罰，結果自動推送至 Discord
- **水星幣** — YouTube 聊天室即時計幣的虛擬經濟系統
- **水星排行** — 即時水星幣排行榜
- **水星伺服器** — 社群法規、公告與歷史檔案
- **互動地圖** — 遊戲地圖頁面
- **管理後台** — 資料庫備份、頻道設定、匿名投稿管理

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
- [CI/CD](#cicd)
- [部署](#部署)
- [專案結構](#專案結構)
- [第三方整合](#第三方整合)
- [常見問題](#常見問題)
- [貢獻指南](#貢獻指南)
- [授權](#授權)

---

## 專案架構

本服務以單一 Rust 二進位檔啟動，於 tokio 執行環境中並行運行三大任務（`src/main.rs`）。任一任務出錯時會記錄錯誤，並於 60 秒後自動重啟；收到 Ctrl+C 訊號時則優雅關機。

```
+--------------------------------------------------------------+
|                        mercury_land                          |
|                     (data/sqlite.db)                         |
+--------------------------------------------------------------+
        |                  |                    |
        v                  v                    v
+---------------+  +----------------+  +---------------------+
|  webpage::run |  |  discord::run  |  |    youtube::run     |
|  actix-web 4  |  | poise/serenity |  | YouTube Data API v3 |
|  HTTP :8080   |  |  Discord 機器人  |  | 直播聊天室監聽 (60s)  |
|  22 條 API    |  |  11 個斜線指令   |  | 水星幣即時計幣       |
+---------------+  +----------------+  +---------------------+
```

---

## 技術棧

### 前端（`web/`）

| 類別        | 技術                                                                      |
| ----------- | ------------------------------------------------------------------------- |
| 框架        | Vue 3.4（`<script setup>` SFC）、TypeScript                               |
| 建置        | Vite 7（`@` 別名指向 `./web`）、vue-tsc                                   |
| 路由        | vue-router 4                                                              |
| UI 框架     | Vuestic UI 1.10（暗色主題）                                               |
| CSS         | Tailwind CSS 3、SCSS、PostCSS                                             |
| HTTP 客戶端 | axios                                                                     |
| 工具庫      | @vueuse/core、@vueuse/components                                          |
| 圖示        | @vicons（Fluent / Ionicons / Tabler / Font Awesome）                      |
| 字型        | vfonts、Google Fonts（Source Sans Pro、Playfair Display、Material Icons） |
| 轉盤        | vue-wheel-spinner                                                         |
| 格式化      | Prettier（tabWidth 4）                                                    |

### 後端（Rust，edition 2024）

| 類別       | 技術                                                          |
| ---------- | ------------------------------------------------------------- |
| Web 框架   | actix-web 4（+ actix-cors、actix-files、actix-multipart）     |
| 資料庫     | rusqlite 0.32（bundled SQLite）+ r2d2 連線池 + sea-query 0.32 |
| Discord    | poise 0.6.1、serenity 0.12.4                                  |
| YouTube    | google-youtube3 6.0、yup_oauth2（Device Flow）、reqwest       |
| 認證       | JWT（HMAC-SHA256）、hmac、sha2                                |
| 非同步執行 | tokio                                                         |
| 其他       | chrono、uuid、regex、once_cell、env_logger                    |

### 資料庫

| 項目     | 內容                                                                 |
| -------- | -------------------------------------------------------------------- |
| 引擎     | SQLite（單一檔案 `data/sqlite.db`）                                  |
| 寫入模式 | WAL（`journal_mode=WAL`、`synchronous=NORMAL`、`busy_timeout=5000`） |
| 連線池   | r2d2，最大 10 條連線                                                 |
| 版本管理 | `PRAGMA user_version`（目前 **v12**），12 個循序遷移檔               |

---

## 功能總覽

### 直播隨選（VOD 資料庫）

收錄歷次直播 VOD，支援新增、編輯、刪除與後設資料管理（日期、連結、標題、標籤、時長）。內建時間計算工具，可快速換算影片時間碼與摘要統計。

### 直播懲罰（Penalty）

以五階段狀態追蹤每場懲罰的執行進度，提供表格總覽、時間軸檢視、統計圖表與詳細內容編輯。編輯器支援圖片上傳、Steam 連結、YouTube 嵌入與自訂語法標籤。

| 狀態碼 | 狀態名稱 | 說明             |
| ------ | -------- | ---------------- |
| 0      | 未生效   | 尚未排入執行     |
| 1      | 未完成   | 已排入但尚未開始 |
| 2      | 進行中   | 正在執行         |
| 3      | 勉強過   | 低標完成         |
| 4      | 已完成   | 圓滿完成         |

### 幸運轉盤（Wheel）

轉盤以加權隨機決定下一場直播的懲罰項目，支援自訂語法（`名稱x權重`）。轉盤結果可送出至 Discord 懲罰頻道，並自動寫入資料庫。

### 水星幣（Mercury Coin）

YouTube 聊天室即時計幣的虛擬經濟系統。觀眾在直播中發言即可獲得水星幣，並可透過 Discord 指令購買加成與加班時數。詳細規則見[水星幣經濟](#水星幣經濟)。

### 水星排行（Leaderboard）

即時水星幣排行榜，以領獎台形式展示前三名，下方列出完整排名與個人統計。

### 水星伺服器（Publication / 法規）

社群法規條文、最新公告、歷史存檔與加入資訊，分頁呈現。

### 互動地圖（Game Map）

嵌入式互動遊戲地圖頁面。

### 其他頁面

聯絡我們、服務條款（ToS）、隱私權政策（Privacy Policy），以及管理員設定頁面（資料庫備份下載、Discord 頻道設定、匿名投稿管理）。

---

## 前端頁面

| 路由           | 頁面        | 說明                                     |
| -------------- | ----------- | ---------------------------------------- |
| `/`            | Welcome     | 歡迎頁（五張投影片）                     |
| `/publication` | Publication | 法規、公告、歷史檔案、加入資訊           |
| `/map`         | GameMap     | 互動遊戲地圖                             |
| `/vod`         | VOD         | 直播隨選與時間計算                       |
| `/penalty`     | Penalty     | 懲罰追蹤（表格、時間軸、統計、細節編輯） |
| `/contact`     | Contact     | 聯絡我們                                 |
| `/wheel`       | Wheel       | 幸運轉盤                                 |
| `/tos`         | ToS         | 服務條款                                 |
| `/privacy`     | Privacy     | 隱私權政策                               |
| `/leaderboard` | Leaderboard | 水星幣排行榜                             |
| `/setting`     | Setting     | 管理員設定（需登入）                     |

---

## API 文件

所有路由皆以 `/api` 開頭，由 actix-web 提供。CORS 僅允許 `http://localhost:5173` 與 `https://mercuryland.pp.ua`（含 www 子網域）來源。管理相關路由需於 `Authorization` 標頭帶上 JWT（Bearer token）。

### 認證相關

| 方法 | 路徑              | 說明                              | 權限 |
| ---- | ----------------- | --------------------------------- | ---- |
| GET  | `/api/ping`       | 健康檢查                          | 公開 |
| POST | `/api/auth/login` | 以 Discord `/auth` 驗證碼換取 JWT | 公開 |
| POST | `/api/auth/tick`  | JWT 續期（每次 1 小時）           | 公開 |

### 轉盤

| 方法 | 路徑                | 說明                   | 權限   |
| ---- | ------------------- | ---------------------- | ------ |
| POST | `/api/wheel/submit` | 送出轉盤結果至 Discord | 管理員 |

### VOD 隨選

| 方法 | 路徑                     | 說明                    | 權限   |
| ---- | ------------------------ | ----------------------- | ------ |
| GET  | `/api/video/list`        | 列出全部 VOD            | 公開   |
| POST | `/api/video/upload-json` | 以 JSON 批次上傳 VOD    | 管理員 |
| POST | `/api/video/insert`      | 新增單筆 VOD            | 管理員 |
| POST | `/api/video/delete`      | 刪除 VOD                | 管理員 |
| POST | `/api/video/update`      | 更新 VOD                | 管理員 |
| POST | `/api/video/metadata`    | 取得／更新 VOD 後設資料 | 管理員 |

### 懲罰

| 方法 | 路徑                          | 說明                 | 權限   |
| ---- | ----------------------------- | -------------------- | ------ |
| GET  | `/api/penalty/list`           | 懲罰列表             | 公開   |
| POST | `/api/penalty/insert`         | 新增懲罰             | 管理員 |
| POST | `/api/penalty/delete`         | 刪除懲罰             | 管理員 |
| POST | `/api/penalty/update`         | 更新懲罰             | 管理員 |
| GET  | `/api/penalty/detail/{id}`    | 取得懲罰細節         | 公開   |
| POST | `/api/penalty/detail/update`  | 更新懲罰細節（HTML） | 管理員 |
| POST | `/api/penalty/history/update` | 更新懲罰歷史紀錄     | 管理員 |

### 排行榜與設定

| 方法 | 路徑                  | 說明                   | 權限   |
| ---- | --------------------- | ---------------------- | ------ |
| GET  | `/api/leaderboard`    | 水星幣排行榜           | 公開   |
| GET  | `/api/setting/backup` | 下載 SQLite 資料庫備份 | 管理員 |
| GET  | `/api/setting/config` | 讀取頻道設定           | 公開   |
| POST | `/api/setting/config` | 更新頻道設定           | 管理員 |

### 圖片

| 方法 | 路徑                    | 說明                       | 權限   |
| ---- | ----------------------- | -------------------------- | ------ |
| POST | `/api/image/upload`     | 上傳圖片                   | 管理員 |
| GET  | `/api/image/get/{name}` | 取得圖片（依 UUIDv5 名稱） | 公開   |

> 管理員判定方式：後端解碼 JWT 取得 Discord 使用者 ID，與 `data/config.json` 中的 `discord.admin` 列表比對。

---

## Discord 指令

機器人以 poise / serenity 實作，支援全域斜線指令（Slash Commands），所有指令與回應皆使用繁體中文。

| 指令                 | 說明                                               |
| -------------------- | -------------------------------------------------- |
| `/auth`              | 產生 8 字元驗證碼（30 分鐘有效），供網站管理員登入 |
| `/coin`              | 查詢自己或指定 YouTube 頻道的水星幣餘額            |
| `/give`              | 管理員贈與水星幣給指定使用者                       |
| `/link`              | 連結 Discord 帳號與 YouTube 頻道（2 小時冷卻）     |
| `/unlink`            | 解除 Discord 與 YouTube 帳號連結                   |
| `/help`              | 顯示所有可用指令與說明                             |
| `/refund close`      | 管理員關閉退款申請                                 |
| `/refund reopen`     | 管理員重新開放退款申請                             |
| `/purchase booster`  | 購買水星幣加成（等級 2–9，成本翻倍遞增）           |
| `/purchase overtime` | 購買直播加班時數（1000 幣 / 小時）                 |
| `/vote nominate`     | 提名投票項目                                       |
| `/vote revoke`       | 撤回提名                                           |
| `/vote deadline`     | 設定投票截止時間                                   |
| `/vote conclude`     | 結算投票結果                                       |
| `/vote clear`        | 清除投票資料                                       |
| `/anonymous create`  | 建立匿名投稿（透過 Modal 輸入內容）                |

---

## 資料庫結構

資料庫 Schema 由 `src/database/migration/` 中的 12 個循序 SQL 遷移檔管理，啟動時透過 `PRAGMA user_version`（目前 v12）比對版本，自動執行未套用的遷移。

> 注意：所有資料表皆**無 FOREIGN KEY 約束**，跨表關聯均為隱式邏輯關係。

### user（使用者與水星幣帳本）

| 欄位         | 型別     | 限制                      | 說明                          |
| ------------ | -------- | ------------------------- | ----------------------------- |
| `id`         | INTEGER  | PRIMARY KEY AUTOINCREMENT | 主鍵                          |
| `youtube`    | TEXT     | UNIQUE, NOT NULL          | YouTube 頻道 ID（主要查詢鍵） |
| `discord`    | BIGINT   | UNIQUE, nullable          | Discord 使用者 ID             |
| `display`    | TEXT     | NOT NULL                  | 顯示名稱                      |
| `coin`       | INTEGER  | NOT NULL                  | 水星幣餘額                    |
| `updated_at` | DATETIME | NOT NULL                  | 最後更新時間                  |

### video（VOD 檔案庫）

| 欄位       | 型別    | 限制                   | 說明                                    |
| ---------- | ------- | ---------------------- | --------------------------------------- |
| `id`       | INTEGER | PRIMARY KEY            | 主鍵                                    |
| `date`     | DATE    | NOT NULL               | 直播日期                                |
| `link`     | TEXT    | NOT NULL, UNIQUE INDEX | 影片連結                                |
| `title`    | TEXT    | NOT NULL               | 影片標題                                |
| `tags`     | TEXT    | NOT NULL               | 標籤（JSON 陣列，如 `["tag1","tag2"]`） |
| `duration` | TEXT    | NOT NULL               | 時長（如 `01:30:00`）                   |

### penalty（懲罰追蹤）

| 欄位      | 型別    | 限制        | 說明                                          |
| --------- | ------- | ----------- | --------------------------------------------- |
| `id`      | INTEGER | PRIMARY KEY | 主鍵                                          |
| `date`    | DATE    | NOT NULL    | 排定日期                                      |
| `name`    | TEXT    | NOT NULL    | 懲罰名稱                                      |
| `detail`  | TEXT    | NOT NULL    | 細節內容（HTML）                              |
| `state`   | INTEGER | NOT NULL    | 狀態碼（0–4，見上方狀態對照表）               |
| `history` | TEXT    | NOT NULL    | 歷史紀錄（JSON 陣列，每項為 `[state, date]`） |

### image（上傳圖片）

| 欄位   | 型別    | 限制             | 說明                              |
| ------ | ------- | ---------------- | --------------------------------- |
| `id`   | INTEGER | PRIMARY KEY      | 主鍵                              |
| `name` | TEXT    | NOT NULL, UNIQUE | UUIDv5 檔名（由圖片內容雜湊產生） |
| `data` | BLOB    | NOT NULL, UNIQUE | 原始圖片位元組                    |
| `mime` | TEXT    | NOT NULL         | MIME 型別（如 `image/png`）       |

### config（系統設定鍵值儲存）

| 欄位   | 型別    | 限制        | 說明           |
| ------ | ------- | ----------- | -------------- |
| `id`   | INTEGER | PRIMARY KEY | 設定鍵（數值） |
| `text` | TEXT    | NOT NULL    | 設定值         |

| 鍵值 | 名稱             | 用途                       |
| ---- | ---------------- | -------------------------- |
| 0    | ChannelPenalty   | 懲罰公告 Discord 頻道 ID   |
| 1    | ChannelCoin      | 水星幣公告 Discord 頻道 ID |
| 2    | ChannelVote      | 投票 Discord 頻道 ID       |
| 3    | MessageVote      | 投票訊息 ID                |
| 4    | YoutubeChannelId | 監聽的 YouTube 頻道 ID     |

### anonymous（匿名投稿）

| 欄位         | 型別     | 限制        | 說明                     |
| ------------ | -------- | ----------- | ------------------------ |
| `id`         | INTEGER  | PRIMARY KEY | 主鍵                     |
| `author`     | BIGINT   | NOT NULL    | 投稿者 Discord 使用者 ID |
| `updated_at` | DATETIME | NOT NULL    | 投稿時間                 |

---

## 水星幣經濟

水星幣由 YouTube 直播聊天室監聽任務即時計幣，資料即時寫入 `user` 表。計幣規則如下：

| 規則             | 一般觀眾 | 頻道會員（贊助者） |
| ---------------- | -------- | ------------------ |
| 每則聊天室訊息   | 1 幣     | 2 幣               |
| 每日首則訊息獎勵 | 10 幣    | 20 幣              |
| 每日計幣上限     | 50 幣    | 100 幣             |

為防止洗幣，每位觀眾每 30 秒最多計入一則訊息。

**加成（Booster）**可用來加倍聊天室計幣，等級與成本如下：

| 加成等級 | 成本（水星幣） |
| -------- | -------------- |
| 2        | 50             |
| 3        | 100            |
| 4        | 200            |
| 5        | 400            |
| 6        | 800            |
| 7        | 1,600          |
| 8        | 3,200          |
| 9        | 6,400          |

**加班時數**：1000 幣可購買 1 小時直播加班。

加成與加班皆透過 Discord 的 `/purchase booster` 與 `/purchase overtime` 指令購買。

---

## 身分驗證

本專案採用輕量自訂認證機制，無需第三方 OAuth：

1. 管理員在 Discord 頻道中執行 `/auth` 指令，機器人產生一組 **8 字元英數驗證碼**（30 分鐘有效，單次使用）。
2. 管理員在網站設定頁面輸入驗證碼，前端取得客戶端 IP 後一併 `POST /api/auth/login`。
3. 後端以 **HMAC-SHA256** 簽發 JWT（有效期 1 小時）回傳前端，前端存入 localStorage。
4. 前端每 10 分鐘自動呼叫 `POST /api/auth/tick` 續期，直到登出或過期。

> JWT 簽署密鑰於每次伺服器啟動時隨機產生——伺服器重啟後所有既有 session 立即失效，需重新登入。

---

## 本地開發

### 前置需求

- **Rust**（edition 2024，stable toolchain）
- **Node.js** 20+ 與 npm
- SQLite 由 rusqlite bundled 提供，無需另外安裝

### 環境變數

在專案根目錄建立 `.env` 檔案：

| 變數            | 說明                                                           |
| --------------- | -------------------------------------------------------------- |
| `DISCORD_TOKEN` | Discord Bot Token                                              |
| `YOUTUBE_TOKEN` | YouTube API OAuth 用戶端密鑰 JSON（yup_oauth2 Device Flow 用） |

### 設定檔

`data/config.json` — 管理員 Discord 使用者 ID 列表：

```json
{
    "discord": {
        "admin": ["你的Discord使用者ID"]
    }
}
```

### 啟動專案

後端（終端機一）：

```sh
cargo run
# 預設監聽 0.0.0.0:8080
```

前端（終端機二）：

```sh
npm install
npm run dev
# Vite dev server，預設 http://localhost:5173
```

本機開發時，前端 API 請求會自動指向 `http://127.0.0.1:8080`（見 `web/composables/utils.ts`），CORS 已允許 `localhost:5173`。

### 建置

```sh
# 前端
npm run build          # 輸出至 dist/
npm run preview        # 預覽建置產物

# 後端
cargo build --release  # 輸出至 target/release/mercury_land
```

### 程式碼格式化

```sh
cargo fmt              # Rust
npm run format         # Vue / TypeScript / CSS（Prettier）
```

---

## CI/CD

本專案使用 GitHub Actions，共三個 workflow（`.github/workflows/`）。

### check.yml（Pull Request 檢查）

PR 合入 `main` 分支前自動執行：

| 檢查項目            | 說明                                          |
| ------------------- | --------------------------------------------- |
| `cargo fmt --check` | Rust 格式化檢查                               |
| `cargo test`        | Rust 單元測試                                 |
| Docker build        | 驗證 Docker 映像檔建置                        |
| `vue-tsc --noEmit`  | Vue TypeScript 型別檢查                       |
| Prettier            | 前端格式化檢查                                |
| `vite build`        | 前端建置驗證                                  |
| 版本一致性          | `Cargo.toml` 與 `package.json` 版本號必須一致 |

### publish.yml（前端發布）

推送至 `main` 分支時自動執行：

1. `npm ci` → `vue-tsc` → `vite build`
2. `wrangler pages deploy dist --project-name=mercuryland` 部署至 **Cloudflare Pages**

### build.yml（後端發布）

僅當 `Cargo.toml` 中的版本號變更時觸發：

1. Docker 多階段建置（`rust:alpine` → `scratch`）
2. `docker save` 匯出映像檔
3. 透過 **cloudflared SSH 隧道**上傳至遠端伺服器
4. 遠端執行 `docker load` + 重啟服務

---

## 部署

### 前端（Cloudflare Pages）

`https://mercuryland.pp.ua` — 推送至 `main` 分支後由 `publish.yml` 自動部署。

### 後端（Docker + Cloudflare Tunnel）

- 多階段 Docker 建置：`rust:alpine` 編譯（musl 靜態鏈結）→ `scratch` 最小執行環境
- 容器暴露 `8080` 連接埠
- 建置時透過 `--build-arg` 注入 `DISCORD_TOKEN` 與 `YOUTUBE_TOKEN`
- 部署流程由 `build.yml` 自動化（cloudflared SSH 隧道 + `docker load` + 遠端重啟腳本）
- API 端點 `https://api.mercuryland.pp.ua` 經 Cloudflare Tunnel 反向代理至容器

---

## 專案結構

```
mercuryland/
├── web/                          # Vue 3 前端原始碼
│   ├── main.ts                   # 應用程式進入點
│   ├── router.ts                 # Vue Router 路由定義（11 條）
│   ├── App.vue                   # 根元件（暗色主題、漢堡選單）
│   ├── components/               # 頁面元件（依功能分資料夾）
│   │   ├── welcome/              # 歡迎頁（五張投影片）
│   │   ├── wheel/                # 幸運轉盤（Spinner + Wheel）
│   │   ├── penalty/              # 懲罰追蹤（表格/時間軸/細節/編輯器）
│   │   ├── vod/                  # VOD 隨選（列表/新增/時間計算）
│   │   ├── publication/          # 法規/公告/歷史存檔/加入資訊
│   │   ├── setting/              # 管理設定（備份/頻道/匿名管理）
│   │   ├── login/                # 登入對話框
│   │   ├── contact/              # 聯絡我們
│   │   └── law/                  # 服務條款 / 隱私權政策
│   ├── composables/              # 組合式函數（axios、authState、penalty、vod、utils）
│   └── assets/
│       └── data/                 # 靜態 JSON 資料（法規文件、地圖存檔、懲罰狀態等）
├── src/                          # Rust 後端原始碼
│   ├── main.rs                   # 三大 tokio 任務入口（webpage / discord / youtube）
│   ├── webpage/                  # actix-web API 伺服器（mod.rs 註冊 22 條路由）
│   │   ├── auth/                 # 登入 / JWT 簽發與驗證
│   │   ├── wheel/                # 轉盤送出
│   │   ├── video/                # VOD CRUD
│   │   ├── penalty/              # 懲罰 CRUD
│   │   ├── leaderboard/          # 排行榜
│   │   ├── setting/              # 設定讀寫 / 資料庫備份
│   │   └── image/                # 圖片上傳與讀取
│   ├── discord/                  # Discord 機器人（poise/serenity 斜線指令）
│   ├── youtube/                  # YouTube 直播聊天室監聽（OAuth Device Flow）
│   ├── coin/                     # 水星幣計幣規則與聊天室指令處理
│   ├── database/                 # SQLite 資料存取層
│   │   ├── migration/            # 12 個循序 SQL 遷移檔（v1 → v12）
│   │   ├── mod.rs                # 連線池與遷移執行
│   │   ├── user.rs               # 使用者 / 水星幣存取
│   │   ├── video.rs              # VOD 存取
│   │   ├── penalty.rs            # 懲罰存取
│   │   ├── image.rs              # 圖片存取
│   │   ├── config.rs             # 系統設定存取
│   │   └── anonymous.rs          # 匿名投稿存取
│   ├── config.rs                 # 應用程式設定（Discord ID、Auth code）
│   └── error.rs                  # 錯誤型別定義
├── styles/                       # 全域 CSS（VaModal 玻璃態樣式、歡迎頁動畫）
├── public/                       # 靜態資源（Google 驗證頁、圖示、音效）
├── data/                         # 執行期資料（sqlite.db、config.json、youtube.secret）
├── scripts/                      # Python 輔助腳本（懲罰歷史計算 / 讀取）
├── .github/workflows/            # CI/CD pipeline（check / publish / build）
├── Dockerfile                    # 多階段 Docker 建置
├── Cargo.toml / Cargo.lock       # Rust 依賴與版本
├── package.json                  # Node 依賴與版本（與 Cargo.toml 同步）
├── vite.config.ts                # Vite 建置設定
├── tailwind.config.js            # Tailwind CSS 設定
└── tsconfig.json                 # TypeScript 設定
```

---

## 第三方整合

| 服務                    | 用途                                                                      |
| ----------------------- | ------------------------------------------------------------------------- |
| **Discord**             | 社群機器人、斜線指令、驗證碼登入、懲罰與金幣公告推播                      |
| **YouTube Data API v3** | 直播頻道偵測、聊天室訊息即時監聽（60 秒輪詢）、OAuth 2.0 Device Flow 授權 |
| **Cloudflare Pages**    | 前端靜態託管與自動部署                                                    |
| **Cloudflare Tunnel**   | SSH 後端部署隧道（cloudflared）                                           |
| **Google Fonts**        | 網頁字型（Source Sans Pro、Playfair Display、Material Icons）             |

---

## 常見問題

### 水星幣怎麼取得？

在 YouTube 直播聊天室中發言即可自動獲得水星幣。一般觀眾每則訊息 1 幣、每日首則 10 幣、每日上限 50 幣；頻道會員（贊助者）加倍。詳細規則見[水星幣經濟](#水星幣經濟)。

### 為什麼伺服器重啟後需要重新登入？

JWT 簽署密鑰於每次伺服器啟動時隨機產生，重啟後所有既有 token 即失效。這是設計上的安全考量——確保沒有任何永久的 admin session。

### 資料庫檔案放在哪裡？

`data/sqlite.db`（WAL 模式，附帶 `-wal` 與 `-shm` 輔助檔）。可透過管理後台下載備份，或使用 `sqlite3` 命令列工具直接查詢。

### 如何成為管理員？

將你的 Discord 使用者 ID 加入 `data/config.json` 的 `discord.admin` 陣列中，重啟伺服器即可生效。

### 前端開發時 API 連不上？

確認後端正以 `cargo run` 執行中（監聽 8080 port）。本機開發時，前端 `BASE_URL` 會自動設為 `http://127.0.0.1:8080`（見 `web/composables/utils.ts`），CORS 亦已允許 `localhost:5173`。

---

## 貢獻指南

1. **Fork** 本專案並建立 Feature Branch
2. 修改後確保通過格式化檢查：`cargo fmt` 與 `npm run format`
3. 若變更涉及版本號，需**同步更新** `Cargo.toml` 與 `package.json`（CI 會自動檢查一致性）
4. 發起 **Pull Request** 至 `main` 分支，CI 將自動執行 `check.yml` 驗證

---

## 授權

本專案採用 [Apache License 2.0](./LICENSE) 釋出。

---

<p align="center">
  <a href="https://mercuryland.pp.ua">水星樂園地球分部</a>
  ·
  <a href="https://github.com/champsing/mercuryland">GitHub</a>
</p>
