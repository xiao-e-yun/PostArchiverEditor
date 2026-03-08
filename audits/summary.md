# Post Archiver Editor 可用性審計總結

> 審計日期：2026-03-08  
> 測試環境：http://192.168.0.100:3000  
> 應用版本：debug build

---

## 總體評估

應用基本可用，主要功能（文章、作者、標籤、收藏集、平台的列表與編輯）皆正常運作。
File Metas 頁籤是目前最需要關注的領域：除了導致頁籤完全無法使用的 **Critical Bug（BUG-001）** 外，
即使修復後仍面臨多個 **High 等級問題**：無法上傳圖片（FEAT-001）、ContentInput 只能選同 Post 既有 FileMeta（FEAT-002）、
修改 `post` 欄位不移動實體檔案造成路徑斷裂（BUG-004）、以及懸空 FileMeta 問題（BUG-005）。

---

## 問題清單

| ID | 類別 | 嚴重程度 | 說明 | 詳情 |
|----|------|---------|------|------|
| BUG-001 | 後端 Bug | 🔴 Critical | `GET /api/file_metas?page=0` 500 錯誤，File Metas 頁籤完全無法使用 | bugs.md |
| BUG-002 | 前端 Bug | 🟡 Minor | 頁面載入時出現 `GET /null` 異常請求 | bugs.md |
| BUG-003 | 後端 Bug | 🟠 High | FileMeta 列表回應缺少 `total` 欄位，分頁計數不正確 | bugs.md |
| BUG-004 | 前後端 Bug | 🟠 High | 修改 FileMeta 的 `post` 欄位不移動實體檔案，導致圖片路徑斷裂 | bugs.md |
| BUG-005 | 後端 Bug | 🟡 Medium | 懸空 FileMeta：移除 Content 中的 File 引用後，FileMeta 記錄殘留 | bugs.md |
| UX-001 | UX | 🟠 Medium | Post 編輯器的 Save/Discard 按鈕在內容較多時超出視窗可見範圍 | ux.md |
| UX-002 | UX | 🟢 Low | 初始畫面沒有預設選中任何項目 | ux.md |
| UX-003 | UX | 🟢 Low | File Metas 頁籤錯誤訊息暴露底層 serde 錯誤 | ux.md |
| UX-004 | UX | 🟢 Low | 側邊欄標題過長被截斷，無 tooltip | ux.md |
| UX-005 | UX | ⚪ Info | Post 編輯器有兩個功能不同的 Save 按鈕，可能混淆 | ux.md |
| FEAT-001 | 功能缺失 | 🟠 High | 無圖片上傳 / 建立新 FileMeta 的 API 和前端入口 | features.md |
| FEAT-002 | 功能缺失 | 🟠 High | ContentInput 只能選取同一 Post 的已有 FileMetas，無法跨 Post 引用或插入新媒體 | features.md |
| FEAT-003 | 功能缺失 | 🟡 Medium | 無孤立 FileMeta 偵測與批次清理工具 | features.md |

---

## 功能狀態

| 模組 | 狀態 |
|------|------|
| Posts — 列表 | ✅ 正常 |
| Posts — 詳情 / 編輯 | ✅ 正常 |
| Authors — 列表 | ✅ 正常 |
| Authors — 詳情 / 編輯 | ✅ 正常 |
| Tags — 列表 | ✅ 正常 |
| Tags — 詳情 / 編輯 | ✅ 正常 |
| Collections — 列表 | ✅ 正常 |
| Collections — 詳情 / 編輯 | ✅ 正常 |
| Platforms — 列表 | ✅ 正常 |
| Platforms — 詳情 / 編輯 | ✅ 正常 |
| File Metas — 列表 | ❌ 500 錯誤 (BUG-001)，修復後尚有 BUG-003 |
| 搜尋（側邊欄） | ✅ 正常 |
| 深色模式切換 | ✅ 正常 |
| URL 狀態路由 | ✅ 正常 |
| 儲存 / 捨棄變更 | ✅ 正常 |
| 無限捲動 + 分頁 | ✅ 正常 |

---

## 詳細報告索引

- [bugs.md](bugs.md) — Bug 報告（BUG-001 ~ BUG-005）
- [ux.md](ux.md) — UX / 可用性問題（UX-001 ~ UX-005）
- [features.md](features.md) — 功能覆蓋審計（含 FileMeta 深度分析、ContentInput 限制）
- [api.md](api.md) — API 網路請求審計
