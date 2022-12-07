$ErrorActionPreference = 'Stop'

$packageName = 'starship'

$url_x86_64_zip = ''
$url_i686_zip = ''
$checksum_x86_64_zip = ''
$checksum_i686_zip = ''

$checksumType = 'sha256'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

Install-ChocolateyZipPackage -PackageName "$packageName" `
    -Url "$url_i686_zip" `
    -Url64 "$url_x86_64_zip" `
    -UnzipLocation "$toolsDir" `
    -Checksum "$checksum_i686_zip" `
    -Checksum64 "$checksum_x86_64_zip" `
    -ChecksumType "$checksumType"

# Add to Profile
Write-Host "Add the following to the end of ~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1 'Invoke-Expression (&starship init powershell)'"
