# Bug 報告

> 審計日期：2026-03-08  
> 測試環境：http://192.168.0.100:3000

---

## BUG-001 [Critical] File Metas 列表 API 500 錯誤

**類別**：後端 / API  
**嚴重程度**：Critical  
**影響範圍**：File Metas 頁籤完全無法使用

### 症狀
- 切換至 `File Metas` 頁籤時，側邊欄顯示紅色錯誤框：
  > Error parsing response  
  > Unexpected token 'c', "can only f"... is not valid JSON
- 瀏覽器 console 出現 `Failed to load resource: the server responded with a status of 500`
- `GET /api/file_metas?page=0 => [500] Internal Server Error`

### 根本原因
`list_file_meta_handler` 回傳型別為 `WithRelations<Vec<FileMeta>>`。  
`WithRelations<T>` 結構中 `inner: T` 欄位標記了 `#[serde(flatten)]`，  
而 serde 只能 flatten **struct / map**，**不能** flatten **sequence（Vec）**。

其他 category 的 list endpoint 使用 `WithRelations<Totalled<Vec<T>>>`，  
`Totalled<T>` 是一個包含 `items: T` 和 `total: u64` 的 struct，因此可以正常 flatten。

```
// src/api/category/file_meta.rs:42
async fn list_file_meta_handler(...) 
  -> Result<Json<WithRelations<Vec<FileMeta>>>, StatusCode>  // ← 有問題
  
// src/api/category/mod.rs:78
async fn list_category_handler<T: Category>(...) 
  -> Result<Json<WithRelations<Totalled<Vec<T>>>>, StatusCode>  // ← 正確
```

伺服器回傳的原始錯誤文字：  
`"can only flatten structs and maps (got a sequence)"`

### 修復建議
將 `list_file_meta_handler` 改為使用 `Totalled<Vec<FileMeta>>`，
或讓 `FileMeta` 實作 `Category` trait 以共用 `list_category_handler`。

---

## BUG-002 [Minor] 頁面載入時出現 `GET /null` 請求

**類別**：前端 / 網路  
**嚴重程度**：Minor  
**影響範圍**：可能造成無效 HTTP 請求，但不影響功能

### 症狀
- 每次頁面初次載入時，網路請求中出現：  
  `[GET] http://192.168.0.100:3000/null => [200] OK`
- 返回 200 是因為 debug 模式下所有路徑都被 reverse proxy 到 Vite，Vite 回傳 SPA index.html。

### 可能原因
前端某處在資料尚未載入時，以 `null` 值建構 URL 字串（e.g., `"/images/..." + null`），
導致瀏覽器或 fetch 請求到 `/null`。需進一步追查 image src 或 fetch URL 的拼接邏輯。

### 修復建議
搜尋所有 URL 拼接與 img src 綁定，確保 null/undefined 值有適當的 guard 防止字串化。

---

## BUG-003 [High] FileMeta 列表缺少 `Totalled` 包裝，無法支援分頁計數

**類別**：後端 / API  
**嚴重程度**：High  
**影響範圍**：File Metas 頁籤分頁/無限捲動功能不完整

### 症狀
即使 BUG-001（序列化錯誤）修復後，`list_file_meta_handler` 仍回傳
`WithRelations<Vec<FileMeta>>` 而非 `WithRelations<Totalled<Vec<FileMeta>>>`。

其他所有 category 列表 API（posts、authors、tags 等）都在 `WithRelations` 中包裝了
`Totalled<Vec<T>>`，回應中帶有 `total: u64` 欄位，前端可藉此知道總筆數：

```json
{ "items": [...], "total": 92, "file_metas": [...] }
```

File Metas 的回應缺少 `total`，前端無法正確的計算總頁數或顯示「共 N 筆」資訊。

### 相關程式碼
```rust
// src/api/category/file_meta.rs:42
async fn list_file_meta_handler(...)
  -> Result<Json<WithRelations<Vec<FileMeta>>>, StatusCode>  // ← 缺少 Totalled
```

### 修復建議
將回傳型別改為 `WithRelations<Totalled<Vec<FileMeta>>>`，
查詢時加入 `SELECT COUNT(*) FROM file_metas` 取得總數，
或讓 `FileMeta` 實作 `Category` trait 以直接使用 `list_category_handler`。

---

## BUG-004 [High] 修改 FileMeta 的 `post` 關聯不移動實體檔案，導致路徑斷裂

**類別**：前端 + 後端  
**嚴重程度**：High  
**影響範圍**：File Metas 詳情編輯，修改所屬 Post 後圖片/檔案 404

### 症狀
`FileMeta.vue` 提供了 `CategoryInput` 元件讓使用者修改 FileMeta 的 `post` 欄位。
然而：

1. **後端問題**：`UpdateFileMetaPayload` 沒有 `post` 欄位，PATCH 請求中的 `post` 欄位會被靜默忽略。
2. **前端問題**：即使後端支援，`getFileMetaPath()` 依賴 `fileMeta.post` 計算磁碟路徑：
   ```typescript
   // frontend/src/utils.ts
   export function getFileMetaPath(fileMeta: FileMeta, raw = false): string {
     const base = isImage ? "images" : "resource";
     return `/${base}/${Math.floor(fileMeta.post / 2048)}/${fileMeta.post % 2048}/${fileMeta.filename}`;
   }
   ```
   修改 `post` 欄位後，資料庫中的路徑計算結果改變，但磁碟上的實體檔案並未移動，
   導致所有 `<img>` src 和下載連結立即變成 404。
3. **importer 行為**：`import_file_meta_with_content` 和 `import_file_meta_by_rename` 才是能安全移動檔案的方法，但這些只供內部 importer 使用，HTTP API 層未暴露。

### 修復建議
**短期**：從 `FileMeta.vue` 移除 `post` 的 `CategoryInput`，防止使用者修改此欄位。  
**長期**：若需要支援此功能，後端需在更新 `post` 欄位時同步移動磁碟上的實體檔案（rename/move），且需為此提供獨立 API，不應由普通 PATCH 觸發。

---

## BUG-005 [Medium] 懸空 FileMeta — 移除 Content 中的 File 引用後，FileMeta 資料庫記錄殘留

**類別**：後端 / 資料完整性  
**嚴重程度**：Medium  
**影響範圍**：長期使用後，資料庫和磁碟上會堆積未被任何 Post content 引用的孤立 FileMeta

### 症狀
在 `ContentInput.vue` 編輯 Post 的 `content` 陣列時，若從清單中刪除一個 `Content::File(id)` 項目，
PATCH 請求只會更新 `post.content`，FileMeta 資料庫記錄和磁碟上的實體檔案不受影響。

`manager/file_meta.rs` 中 `delete()` 的文件明確說明：
> "But it will not delete post.content related to this file."

這是雙向問題：
- 刪除 FileMeta 不清理 Post.content 中對其的引用（導致 content `null` 或 404）
- 刪除 Post.content 中的引用不清理對應 FileMeta（導致孤立 FileMeta）

懸空條件更廣泛包含：
1. 編輯 Post content，移除 File 區塊 → FileMeta 殘留
2. 刪除整個 Post → 資料庫有 CASCADE DELETE 嗎？（待確認）
3. 替換 Post / Author / Collection 的縮圖 → 舊縮圖 FileMeta 殘留

### 修復建議
1. **偵測**：新增 `GET /api/file_metas/orphans` endpoint，
   以 `LEFT JOIN` 查詢未被 post.content 或任何 thumb 欄位引用的 FileMeta。
2. **清理**：新增 `DELETE /api/file_metas/orphans` 批次刪除接口，並在前端 File Metas 頁籤提供「掃描孤立檔案」按鈕。
3. **預防**：考慮在後端 PATCH post 時，偵測 content diff，對移除的 File 項目自動刪除 FileMeta（需評估是否為預期行為）。
