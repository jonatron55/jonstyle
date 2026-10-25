[CmdletBinding()]
param(
    [Parameter()]
    [string]$Source = ".\themes",

    [Parameter()]
    [string]$Out = ".\out",

    [Parameter()]
    [switch]$Recurse
)
$Root = Split-Path -Path $PSScriptRoot -Parent
Push-Location -Path $Root

try {
    cargo build --release --bin mktheme

    $themes = if ($Recurse) {
        Get-ChildItem -Path $Source -Recurse -Filter *.toml
    }
    else {
        Get-ChildItem -Path $Source -Filter *.toml
    }

    $themes | ForEach-Object {
        .\target\release\mktheme.exe pal $_.FullName --force --output $Out
    }
}
finally {
    Pop-Location
}
