#!/usr/bin/env pwsh

# Starship assumes UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
function global:prompt {
    $out = $null
    # @ makes sure the result is an array even if single or no values are returned
    $jobs = @(Get-Job | Where-Object { $_.State -eq 'Running' }).Count

    $lwd = (Get-Location)
    $cwd = (Convert-Path $lwd)

    if ($lastCmd = Get-History -Count 1) {
        $duration = [math]::Round(($lastCmd.EndExecutionTime - $lastCmd.StartExecutionTime).TotalMilliseconds)
        # & ensures the path is interpreted as something to execute
        $out = @(&::STARSHIP:: prompt "--path=$cwd" "--logical-path=$lwd" --status=$lastexitcode --jobs=$jobs --cmd-duration=$duration)
    } else {
        $out = @(&::STARSHIP:: prompt "--path=$cwd" "--logical-path=$lwd" --status=$lastexitcode --jobs=$jobs)
    }

    # Convert stdout (array of lines) to expected return type string
    # `n is an escaped newline
    $out -join "`n"
}

$ENV:STARSHIP_SHELL = "powershell"
