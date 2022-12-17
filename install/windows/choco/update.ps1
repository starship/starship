#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 3.0

if ($null -eq $ENV:STARSHIP_VERSION) {
    Write-Host "Version is required"
    exit 1
}

$version = "$ENV:STARSHIP_VERSION"
$versionNumber = $version.TrimStart("v")

[xml]$nuspec_file = Get-Content -Path ./install/windows/choco/starship.nuspec
$nuspec_file.package.metadata.version = $versionNumber

$changelog = (Get-Content -Path ./CHANGELOG.md | Out-String)
$nuspec_file.package.metadata.releaseNotes = $changelog

$nuspec_file.Save("./starship.nuspec")

$url_x86_64_zip = "https://github.com/starship/starship/releases/download/$version/starship-x86_64-pc-windows-msvc.zip"
$url_i686_zip = "https://github.com/starship/starship/releases/download/$version/starship-i686-pc-windows-msvc.zip"

$checksum_x86_64_zip = Get-FileHash -Algorithm SHA256 -Path "./starship-x86_64-pc-windows-msvc/starship-x86_64-pc-windows-msvc.zip" | Select-Object -ExpandProperty Hash
$checksum_i686_zip = Get-FileHash -Algorithm SHA256 -Path "./starship-i686-pc-windows-msvc/starship-i686-pc-windows-msvc.zip" | Select-Object -ExpandProperty Hash

if (!(Test-Path "./tools")) {
    New-Item -ItemType Directory -Path "./tools"
}

Get-Content ./install/windows/choco/chocolateyInstall.ps1 | ForEach-Object {
    if ($_ -match '^\$url_x86_64_zip = (.*)') {
        "`$url_x86_64_zip = '$url_x86_64_zip'"
    }
    elseif ($_ -match '^\$url_i686_zip = (.*)') {
        "`$url_i686_zip = '$url_i686_zip'"
    }
    elseif ($_ -match '^\$checksum_x86_64_zip = (.*)') {
        "`$checksum_x86_64_zip = '$checksum_x86_64_zip'"
    }
    elseif ($_ -match '^\$checksum_i686_zip = (.*)') {
        "`$checksum_i686_zip = '$checksum_i686_zip'"
    }
    else {
        $_
    }
} | Set-Content ./tools/chocolateyInstall.ps1

choco pack ./starship.nuspec

if ($null -ne $Env:PUSH_TOKEN) {
    choco push starship.$versionNumber.nupkg --key $ENV:PUSH_TOKEN 
}
else {
    Write-Host "No API key provided, skipping push"
}
