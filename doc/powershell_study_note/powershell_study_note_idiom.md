# PowerShell 勉強メモ イディオム

## ハッシュ化

```powershell
Get-FileHash
```

## base64 エンコード / デコード

```powershell
$Text = 'Hello, World!'
$Bytes = [System.Text.Encoding]::UTF8.GetBytes($Text)
$EncodedText = [Convert]::ToBase64String($Bytes)
$EncodedText

$EncodedText = 'SGVsbG8sIFdvcmxkIQ=='
$Bytes = [Convert]::FromBase64String($EncodedText)
$DecodedText = [System.Text.Encoding]::UTF8.GetString($Bytes)
$DecodedText
```

```powershell
function Get-Base64FromText($Text) { [Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($Text)) }
function Get-TextFromBase64($Base64Text) { [System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($Base64Text)) }
```
