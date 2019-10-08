#!/usr/bin/env pwsh

# Starship assumes UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
function global:prompt {
    $out = $null
    $jobs = @(Get-Job | Where-Object { $_.State -eq 'Running' }).Count

    if ($lastCmd = Get-History -Count 1) {
        $duration = [math]::Round(($lastCmd.EndExecutionTime - $lastCmd.StartExecutionTime).TotalSeconds)
        $out = @(&::STARSHIP:: prompt --status=$lastexitcode --jobs=$jobs --cmd-duration=$duration)
    }
    else {
        $out = @(&::STARSHIP:: prompt --status=$lastexitcode --jobs=$jobs)
    }

    $out -join "`n"
}

$ENV:STARSHIP_SHELL = "powershell"
