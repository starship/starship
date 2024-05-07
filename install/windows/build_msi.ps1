#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 3.0

if ($null -eq $ENV:STARSHIP_VERSION) {
    Write-Host "Version is required"
    exit 1
}

if ($null -eq $ENV:STARSHIP_TARGET) {
    Write-Host "Target is required"
    exit 1
}
 
if (!(Get-Command -Name "wix.exe" -ErrorAction SilentlyContinue)) {
    Write-Host "WiX Toolset is required, Installing..."
    dotnet.exe tool install --global wix
    wix.exe extension add --global WixToolset.UI.wixext
}

$target = "$ENV:STARSHIP_TARGET"
$version = "$ENV:STARSHIP_VERSION"
$versionNumber = $version.TrimStart("v")

$arch = $target.Split('-')[0]
$wix_style_arch = @{
    "x86_64"  = "x64"
    "i686"    = "x86"
    "aarch64" = "arm64"
}[ $arch ]

wix.exe build -ext WixToolset.UI.wixext `
    -culture en-us `
    -pdbtype none `
    -arch $wix_style_arch `
    -define Version=$versionNumber `
    -out target/$target/release/starship-$target.msi `
    ./install/windows/main.wxs
