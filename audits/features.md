# 功能覆蓋審計

> 審計日期：2026-03-08  
> 測試環境：http://192.168.0.100:3000

---

## 頁籤覽覽

| 頁籤 | 列表載入 | 選取 / 詳情 | 編輯 | 儲存 | 刪除 | 搜尋 |
|------|---------|------------|------|------|------|------|
| Posts | ✅ | ✅ | ✅ | ✅ | ✅ (按鈕存在) | ✅ |
| Authors | ✅ | ✅ | ✅ | ✅ | ✅ (按鈕存在) | ✅ |
| Tags | ✅ | ✅ | ✅ | ✅ | ✅ (按鈕存在) | ✅ |
| Collections | ✅ | ✅ | ✅ | ✅ | ✅ (按鈕存在) | ✅ |
| Platforms | ✅ | ✅ | ✅ | ✅ | ✅ (按鈕存在) | ✅ |
| File Metas | ❌ 500 Error | — | — | — | — | — |

---

## Posts（文章）

### 列表
- 以 20 筆為一頁，支援無限捲動（infinite scroll + virtual list）。
- 列表項目顯示：標題、ID、平台標籤（如 `:fanbox`）、縮圖（若有）。
- 搜尋框支援 250ms debounce，即時過濾；搜尋值存於 sessionStorage。
- URL 狀態：`/?tab=posts`（切換頁籤）、`/?item=p-{id}`（選取項目）。

### 詳情 / 編輯
編輯表單分為左右兩欄（lg+ 視窗寬度）：
- **左欄**（主要內容）：
  - Title 輸入框
  - ContentInput（文章 body，含文字段落 + 媒體檔案）
  - ActionButtons（Delete / Discard / Save）
- **右欄**（metadata，sticky 吸頂）：
  - 封面圖選擇器（ImageInput）
  - Title 輸入框（mobile 版，`lg:hidden`）
  - 來源 URL
  - Published / Updated 日期時間（DateTimeInput）
  - Authors（多選 combobox，支援搜尋）
  - Tags（多選 combobox，支援搜尋）
  - Collections（多選 combobox，支援搜尋）
  - Platform（單選 combobox）

### 儲存機制
- 異動偵測：透過 `proxyed.changes` 追蹤 diff，有變更才顯示 `Save` / `Discard` 按鈕。
- 點擊 Save → 觸發 PATCH 請求。
- 點擊 Discard → 清空 `changes`，還原成原始值。
- ContentInput 有獨立的 Save 按鈕，單獨送出 content 變更。

---

## Authors（作者）

### 列表
- 以 20 筆為一頁，支援無限捲動。
- 顯示：名稱、ID、作者頭像縮圖。

### 詳情 / 編輯
- 大型頭像預覽（h-48）
- 名稱文字框
- Thumb 縮圖選擇（下拉 combobox，可預覽 fileMeta）
- Delete 按鈕（條件顯示 Save / Discard）

---

## Tags（標籤）

### 列表
- 2 筆（`free`、`r-18`），以 ID 排序。

### 詳情 / 編輯
- 名稱文字框
- Platform 單選（可指定標籤所屬平台）
- Delete 按鈕

---

## Collections（收藏集）

### 列表
- 6 筆，含縮圖。

### 詳情 / 編輯
- 大型縮圖預覽
- 名稱文字框
- 來源 URL
- Thumb 縮圖選擇
- Delete 按鈕

---

## Platforms（平台）

### 列表
- 3 筆（`unknown`、`fanbox`、`pixiv`）

### 詳情 / 編輯
- 僅有名稱文字框
- Delete 按鈕

---

## File Metas（檔案元資料）— 深度分析

> 此區塊於 2026-03-08 因 BUG-001 修復後的後續問題分析而補充。

### 列表（BUG-001 修復後預期狀況）
- 即使列表 API 正常，回傳的 JSON 缺少 `total` 欄位（見 BUG-003），無限捲動終止條件可能不正確。
- 列表項目顯示：檔名、ID、縮圖（若 mime 為 image）。
- ❌ 無搜尋支援（`list_file_meta_handler` 未實作 `search` query param）。
- ❌ 無建立/上傳新 FileMeta 的入口（見 FEAT-001）。

### 詳情 / 編輯（BUG-001 修復後）
- ✅ 可預覽圖片（image mime）或顯示下載按鈕（非圖片）。
- ✅ 可編輯 `mime` 欄位（`MimeInput`）。
- ✅ 可編輯 `extra` JSON 欄位（`JsonEditor`）。
- ⚠️ `post` 欄位透過 `CategoryInput` 顯示並允許修改，但此修改會導致路徑斷裂（見 BUG-004）。
- ❌ 無法修改 `filename`（顯示但唯讀）。
- ❌ 無法上傳/替換實體檔案（見 FEAT-001）。
- ✅ Delete 按鈕（可刪除記錄，但不一定刪除磁碟檔案——需確認後端實作）。

### 缺失功能總覽

| 功能 | 狀態 | 說明 |
|------|------|------|
| 圖片上傳 | ❌ 缺失 | 後端無 multipart upload endpoint |
| 新增 FileMeta | ❌ 缺失 | 無 POST /api/file_metas |
| Content 中插入新檔案 | ❌ 受限 | 只能從同一 Post 的已有 FileMetas 中選取 |
| 孤立 FileMeta 偵測 | ❌ 缺失 | 無 orphan 查詢 API |
| 孤立 FileMeta 清理 | ❌ 缺失 | 無批次刪除 API |
| FileMeta 搜尋 | ❌ 缺失 | 列表無 search param |
| 安全更改 post 關聯 | ❌ 有誤 | 更改 post 不移動實體檔案（BUG-004）|
| FileMeta 總數顯示 | ❌ 缺失 | 列表缺少 total 欄位（BUG-003）|

---

## ContentInput 中的 FileMeta 支援

`ContentInput.vue` 是 Post 編輯器中處理媒體內容的核心元件，目前存在以下限制：

### 現有行為
- 透過 `FileMetaInput` 可在 content 中選取已存在的 FileMeta。
- `FileMetaSelect.vue` 中：
  ```typescript
  const matchesPost = props.post ? fm.post === props.post : true;
  ```
  **只顯示屬於同一 Post 的 FileMeta**，跨 Post 引用完全被過濾掉。

### 問題
1. **無法上傳新圖片**：`ContentInput` 沒有 `<input type="file">` 或拖拽上傳入口，只能選取既有 FileMeta。
2. **無法引用其他 Post 的媒體**：FileMeta 被強制過濾為 `fm.post === post`，若來源圖片屬於不同 Post，無法選取。
3. **無法在 Content 中新增 FileMeta 項目**：目前只能修改已有 `Content::File(id)` 的 id，無法插入全新 File 區塊（UI 中未提供「新增媒體區塊」按鈕）。
- 功能最精簡

---

## File Metas（檔案元資料）

- **完全無法使用**，後端 API 回傳 500 錯誤（詳見 bugs.md BUG-001）。
- 前端正確顯示錯誤提示框（Alert variant="destructive"），錯誤處理機制運作正常。

---

## 全局功能

### 深色模式
- Header 右側有主題切換按鈕（太陽 / 月亮圖示）。
- 點擊即時切換 Light / Dark 模式，全頁套用。
- ✅ 運作正常。

### 通知系統
- `region "Notifications alt+T"` 存在於頁面，可以 `alt+T` 觸發。
- 未發現通知顯示場景（測試期間無 toast/通知出現）。

### URL 路由狀態保留
- 頁籤與選取項目狀態存於 query params，重新整理後可還原。
- 格式：`/?item=p-38&tab=authors`
