# API 網路請求審計

> 審計日期：2026-03-08  
> 測試環境：http://192.168.0.100:3000

---

## 請求記錄

以下為完整測試流程中所有觀測到的 HTTP 請求：

| 方法 | 路徑 | 狀態碼 | 說明 |
|------|------|--------|------|
| GET | `/null` | 200 | ⚠️ 異常請求，詳見下方 |
| GET | `/api/posts?page=0` | 200 | 文章列表第 1 頁 |
| GET | `/api/posts/38` | 200 | 取得 Post #38 詳情 |
| GET | `/api/posts?page=1` | 200 | 文章列表第 2 頁 |
| GET | `/api/posts?page=2` | 200 | 文章列表第 3 頁 |
| GET | `/api/posts?page=3` | 200 | 文章列表第 4 頁 |
| GET | `/api/posts?page=4` | 200 | 文章列表第 5 頁（空白，結束分頁） |
| GET | `/api/authors?page=0` | 200 | 作者列表 |
| GET | `/api/authors/1` | 200 | 作者 #1 詳情 |
| GET | `/api/tags?page=0` | 200 | 標籤列表 |
| GET | `/api/tags/1` | 200 | 標籤 #1 詳情 |
| GET | `/api/collections?page=0` | 200 | 收藏集列表 |
| GET | `/api/collections/1` | 200 | 收藏集 #1 詳情 |
| GET | `/api/platforms?page=0` | 200 | 平台列表 |
| GET | `/api/platforms/1` | 200 | 平台 #1 詳情 |
| GET | `/api/file_metas?page=0` | **500** | ❌ 伺服器內部錯誤 |
| GET | `/api/posts?page=0` | 200 | 切換回 Posts 頁籤，重新載入列表 |
| GET | `/api/posts?search=Lucy&page=0` | 200 | 搜尋包含 "Lucy" 的文章 |
| GET | `/api/posts/63` | 200 | 取得 Post #63 詳情 |

---

## API Response 結構

### 列表端點（正常）
```json
{
  "items": [...],
  "total": 92,
  "file_metas": [...],
  "platforms": [...]
}
```
前端透過 `WithRelations<Totalled<Vec<T>>>` 解析，relations 欄位（file_metas、platforms 等）
扁平化（flatten）在同一層。

### 列表端點（file_metas，異常）
回應 HTTP 500，body 為純文字：
```
can only flatten structs and maps (got a sequence)
```
此為 serde 在嘗試 `#[serde(flatten)]` 序列化 `Vec<FileMeta>` 時拋出的執行時錯誤。

---

## 異常請求分析

### `GET /null`
- 狀態碼：200（debug 模式下 Vite reverse proxy 回傳 SPA index.html）
- 發生時機：頁面初始載入時
- 可能原因：某處圖片 `src` 或 `fetch` URL 以 `null` 字串建構
- 影響：無功能影響，但為潛在問題

---

## API 設計觀察

### 分頁
- 使用 `?page=N` query param（0-based）
- 每頁固定 20 筆（前端以此判斷是否還有下一頁）

### 搜尋
- 使用 `?search={keyword}` query param
- Server-side 過濾，支援各 category 的名稱搜尋

### Relations（關聯資料內嵌）
- API 回應將關聯資料（如 file_metas, platforms）與主資料合併在同一 JSON 物件中
- 前端以 Map 結構建立索引，供各元件查詢使用

### 路由設計
- `/api/posts` + `/api/posts/{id}` 支援 GET / PATCH / DELETE（標準 REST）
- File Metas 未使用共用 `Category` trait，而是手寫 handler，導致實作差異造成錯誤

---

## FileMeta API 缺口分析

### 現有端點
| 方法 | 路徑 | 用途 | 問題 |
|------|------|------|------|
| GET | `/api/file_metas?page=N` | FileMeta 列表 | ❌ 500（BUG-001）；修復後仍缺 `total`（BUG-003）；無 `search` 支援 |
| GET | `/api/file_metas/{id}` | 取得單筆 FileMeta | ✅ 正常 |
| PATCH | `/api/file_metas/{id}` | 更新 mime / extra | ⚠️ 不支援 `post` / `filename` 修改；修改 `post` 會斷裂路徑（BUG-004）|
| DELETE | `/api/file_metas/{id}` | 刪除 FileMeta | ⚠️ 不清理 post.content 中的引用（BUG-005）|

### 缺少端點
| 方法 | 建議路徑 | 用途 |
|------|----------|------|
| POST | `/api/file_metas` | 上傳新檔案，建立 FileMeta（multipart/form-data，含 post、filename、mime）|
| PUT | `/api/file_metas/{id}/content` | 替換實體檔案內容（不改 metadata）|
| GET | `/api/file_metas/orphans` | 查詢未被任何 post.content 或 thumb 欄位引用的孤立 FileMeta |
| DELETE | `/api/file_metas/orphans` | 批次刪除孤立 FileMeta（含磁碟檔案）|
