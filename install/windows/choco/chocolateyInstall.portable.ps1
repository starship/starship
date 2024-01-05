$ErrorActionPreference = 'Stop'

$packageName = $env:ChocolateyPackageName

$url_x86_64_zip = ''
$url_i686_zip = ''
$checksum_x86_64_zip = ''
$checksum_i686_zip = ''

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

# Add to Profile
Write-Host "Add the following to the end of ~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1 'Invoke-Expression (&starship init powershell)'"
