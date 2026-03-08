# UX / 可用性審計

> 審計日期：2026-03-08  
> 測試環境：http://192.168.0.100:3000

---

## UX-001 [Medium] Save / Discard 按鈕在有 Content 的 Post 中超出視窗

**頁面**：Post 詳情編輯  
**嚴重程度**：Medium

### 描述
Post 編輯器左欄結構為：`Title → ContentInput → ActionButtons`。  
`ContentInput` 會依文章內容（文字 + 圖片）動態展開高度。  
當文章包含一張或多張大圖時，ActionButtons（Delete / Save / Discard）會被推到視窗可見範圍以外，
使用者需手動往下捲動才能看到並點擊儲存按鈕。

`Save` 按鈕在未進行任何修改前不會顯示（由 `v-if="isChanged"` 控制），
因此使用者填完表單後往往不知道需要往下捲動才能找到 Save 按鈕。

### 量測
- Save / Discard 按鈕的 `top` 值為 **661px**（視窗高度約為 615px），超出視口 46px 以上。

### 建議
1. 將 ActionButtons 改為 **sticky 定位**（`sticky bottom-0`），始終顯示在視窗底部。
2. 或將 ActionButtons 移至右欄 metadata 區塊的頂部或底部（右欄已為 `sticky top-0`）。

---

## UX-002 [Low] 初始畫面沒有選中任何項目

**頁面**：首頁 / 任何頁籤  
**嚴重程度**：Low

### 描述
初始進入頁面時，主區域顯示 `"Select an item from aside."`，  
沒有預設選中第一筆資料，使用者需手動點擊才能開始編輯。

### 建議
考量 UX，首次載入或切換頁籤時自動選中列表第一筆。

---

## UX-003 [Low] File Metas 頁籤錯誤提示不夠友善

**頁面**：File Metas  
**嚴重程度**：Low

### 描述
當 `/api/file_metas?page=0` 回傳 500 錯誤時，側邊欄顯示紅色 Alert：  
> Error parsing response  
> Unexpected token 'c', "can only f"... is not valid JSON

訊息暴露了 serde 序列化的底層錯誤文字，對一般使用者不夠友善。  
（此問題的根本原因詳見 bugs.md BUG-001）

### 建議
後端應正確處理此情況，或至少回傳格式正確的 JSON 錯誤回應。

---

## UX-004 [Low] 側邊欄標題過長時被截斷

**頁面**：Posts 列表  
**嚴重程度**：Low

### 描述
側邊欄列表項目標題使用 `truncate` CSS class，超出寬度後以 `...` 截斷。  
部分文章標題（如日文長標題）被截斷後難以辨識。

範例：`「千夏サービス?Chinatsu Maid ...」`、`「【全体公開】アニメ「ボクの...」`

### 建議
考慮 hover tooltip 顯示完整標題，或增加側邊欄最小寬度。

---

## UX-005 [Info] ContentInput 有獨立的 Save 按鈕

**頁面**：Post 詳情編輯  
**嚴重程度**：Info（觀察）

### 描述
文章 body（ContentInput）的儲存與外部 ActionButtons 的 Save 相互獨立：
- ContentInput 內部有一個懸浮的 "Save" 按鈕（由 `v-if="changed"` 控制），
  只儲存 content 序列的變更。
- ActionButtons 的 Save 儲存其他 metadata（title, thumb, dates, etc.）。

兩個 Save 按鈕作用不同，但外觀類似，可能混淆使用者。

### 建議
可考慮合併為單一儲存流程，或明確標示兩個 Save 的差異（如：「儲存內容」vs「儲存資訊」）。
