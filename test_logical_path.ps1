function test ($Path, $LogicalPath) {
    "Testing: starship prompt --path '$Path' --logical-path '$LogicalPath'"
    target\debug\starship.exe prompt --path $Path --logical-path $LogicalPath
    "`n"
}

New-PSDrive -Name Sys32 -PSProvider FileSystem -Root 'C:\Windows\System32' -ErrorAction SilentlyContinue | Out-Null
New-PSDrive -Name Dev -PSProvider FileSystem -Root 'C:\Users\Ben Fox\Dev' -ErrorAction SilentlyContinue | Out-Null

test 'C:\Users\Ben Fox\Dev\starship' 'C:\Users\Ben Fox\Dev\starship'
test 'C:\Users\Ben Fox\Dev\starship' 'Dev:\starship'
test 'C:\Users\Ben Fox\Dev\starship' 'whatever\basically'
test 'C:\Windows\System32\' 'C:\Windows\System32\'
test 'C:\Windows\System32\' 'Sys32:\'
test 'HKEY_LOCAL_MACHINE\SOFTWARE' 'HKLM:\SOFTWARE'