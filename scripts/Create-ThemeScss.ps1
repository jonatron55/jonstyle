[CmdletBinding()]
param(
    [Parameter()]
    [string]$Source = ".\themes",

    [Parameter()]
    [string]$Out = ".\scss\themes\",

    [Parameter()]
    [switch]$Recurse
)
$Root = Split-Path -Path $PSScriptRoot -Parent
Push-Location -Path $Root

try {
    cargo build --release

    $themes = if ($Recurse) {
        Get-ChildItem -Path $Source -Recurse -Filter *.toml
    } else {
        Get-ChildItem -Path $Source -Filter *.toml
    }

    $themes | ForEach-Object {
        .\target\release\mktheme.exe apply $_.FullName .\templates\theme-mixin.scss --force --output .\scss\themes
    }
}
finally {
    Pop-Location
}
