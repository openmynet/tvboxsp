# 帮助

## 生成方式

```bash
yarn tauri signer generate -w ./key/app.key

```

## build release

Windows

```powershell
$env:TAURI_PRIVATE_KEY=""
$env:TAURI_KEY_PASSWORD=""

yarn tauri build

```

linux:

```bash
export TAURI_PRIVATE_KEY=""
export TAURI_KEY_PASSWORD=""

yarn tauri build

```
