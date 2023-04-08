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

# Create variant nuspec files
$nuspec_file.package.metadata.id = "starship.portable"
$nuspec_file.Save("./starship.portable.nuspec")

$nuspec_file.package.metadata.id = "starship.install"
$nuspec_file.Save("./starship.install.nuspec")

# Have metapackage depend on starship.install
$nuspec_file.package.metadata.id = "starship"

$deps = $nuspec_file.createelement("dependencies")
$dep = $nuspec_file.createelement("dependency")
$dep.SetAttribute("id", "starship.install")
$dep.SetAttribute("version", "[$versionNumber]")
$deps.AppendChild($dep)
$nuspec_file.package.metadata.AppendChild($deps)
$nuspec_file.Save("./starship.nuspec")

$url_x86_64_zip = "https://github.com/starship/starship/releases/download/$version/starship-x86_64-pc-windows-msvc.zip"
$url_i686_zip = "https://github.com/starship/starship/releases/download/$version/starship-i686-pc-windows-msvc.zip"
$url_x86_64_msi = "https://github.com/starship/starship/releases/download/$version/starship-x86_64-pc-windows-msvc.msi"
$url_i686_msi = "https://github.com/starship/starship/releases/download/$version/starship-i686-pc-windows-msvc.msi"

$checksum_x86_64_zip = Get-FileHash -Algorithm SHA256 -Path "./starship-x86_64-pc-windows-msvc.zip/starship-x86_64-pc-windows-msvc.zip" | Select-Object -ExpandProperty Hash
$checksum_i686_zip = Get-FileHash -Algorithm SHA256 -Path "./starship-i686-pc-windows-msvc.zip/starship-i686-pc-windows-msvc.zip" | Select-Object -ExpandProperty Hash
$checksum_x86_64_msi = Get-FileHash -Algorithm SHA256 -Path "./starship-x86_64-pc-windows-msvc.msi/starship-x86_64-pc-windows-msvc.msi" | Select-Object -ExpandProperty Hash
$checksum_i686_msi = Get-FileHash -Algorithm SHA256 -Path "./starship-i686-pc-windows-msvc.msi/starship-i686-pc-windows-msvc.msi" | Select-Object -ExpandProperty Hash

if (Test-Path "./tools") {
    Remove-Item -Path "./tools" -Recurse -Force
}
New-Item -ItemType Directory -Path "./tools"

# Pack the metapackage as-is without install script
choco pack ./starship.nuspec

foreach ($install_type in @('portable', 'install')) {
    Get-Content ./install/windows/choco/chocolateyInstall.$install_type.ps1 | ForEach-Object {
        if ($_ -match '^\$url_x86_64_zip = (.*)') {
            "`$url_x86_64_zip = '$url_x86_64_zip'"
        }
        elseif ($_ -match '^\$url_i686_zip = (.*)') {
            "`$url_i686_zip = '$url_i686_zip'"
        }
        elseif ($_ -match '^\$url_x86_64_msi = (.*)') {
            "`$url_x86_64_msi = '$url_x86_64_msi'"
        }
        elseif ($_ -match '^\$url_i686_msi = (.*)') {
            "`$url_i686_msi = '$url_i686_msi'"
        }
        elseif ($_ -match '^\$checksum_x86_64_zip = (.*)') {
            "`$checksum_x86_64_zip = '$checksum_x86_64_zip'"
        }
        elseif ($_ -match '^\$checksum_i686_zip = (.*)') {
            "`$checksum_i686_zip = '$checksum_i686_zip'"
        }
        elseif ($_ -match '^\$checksum_x86_64_msi = (.*)') {
            "`$checksum_x86_64_msi = '$checksum_x86_64_msi'"
        }
        elseif ($_ -match '^\$checksum_i686_msi = (.*)') {
            "`$checksum_i686_msi = '$checksum_i686_msi'"
        }
        else {
            $_
        }
    } | Set-Content ./tools/chocolateyInstall.ps1
    
    choco pack ./starship.$install_type.nuspec
}

if ($null -ne $ENV:PUSH_TOKEN) {
    choco push starship.portable.$versionNumber.nupkg --key $ENV:PUSH_TOKEN
    choco push starship.install.$versionNumber.nupkg --key $ENV:PUSH_TOKEN
    choco push starship.$versionNumber.nupkg --key $ENV:PUSH_TOKEN
}
else {
    Write-Host "No API key provided, skipping push"
}
