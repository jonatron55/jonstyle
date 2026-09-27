
$Root = Split-Path -Path $PSScriptRoot -Parent
Push-Location -Path $Root

try {
    cargo build --release

    Get-ChildItem -Path .\themes | ForEach-Object {
        .\target\release\mktheme.exe $_.FullName apply .\templates\theme-mixin.scss --force --output .\scss\themes
    }
}
finally {
    Pop-Location
}
