$ErrorActionPreference = 'Stop'

$packageName = $env:ChocolateyPackageName

$url_x86_64_zip = ''
$url_i686_zip = ''
$url_x86_64_msi = ''
$url_i686_msi = ''
$checksum_x86_64_zip = ''
$checksum_i686_zip = ''
$checksum_x86_64_msi = ''
$checksum_i686_msi = ''

if ("starship.portable" -eq $packageName) {
    $toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
    $packageArgs = @{
        packageName   = $packageName
        fileType      = 'zip'
        url           = $url_i686_zip
        url64bit      = $url_x86_64_zip
        checksum      = $checksum_i686_zip
        checksum64    = $checksum_x86_64_zip
        checksumType  = 'sha256'
        unzipLocation = $toolsDir
    }
    Install-ChocolateyZipPackage @packageArgs
}
elseif ("starship.install" -eq $packageName) {
    $packageArgs = @{
        packageName    = $packageName
        fileType       = 'msi'
        url            = $url_i686_msi
        url64bit       = $url_x86_64_msi
        checksum       = $checksum_i686_msi
        checksum64     = $checksum_x86_64_msi
        checksumType   = 'sha256'
        softwareName   = 'starship*'
        silentArgs     = "/qn /norestart /l*v `"$($env:TEMP)\$($packageName).$($env:chocolateyPackageVersion).MsiInstall.log`""
        validExitCodes = @(0, 3010, 1641)
    }
    Install-ChocolateyPackage @packageArgs
}
elseif ("starship" -eq $packageName) {
    # metadata only package
    exit 0
}
else {
    Write-Host "Unknown package name: $packageName"
    exit 1
}

# Add to Profile
Write-Host "Add the following to the end of ~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1 'Invoke-Expression (&starship init powershell)'"
