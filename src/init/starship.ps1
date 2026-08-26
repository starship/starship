#!/usr/bin/env pwsh

# Replace an earlier initialization cleanly. In particular, this runs the old
# module's OnRemove hook so no renderer or pump subscription survives a reload.
Get-Module -Name starship | Remove-Module -Force

# Keep Starship's implementation out of the global namespace. Only the prompt
# function and the two public transience controls escape this dynamic module.
$null = New-Module starship {
    $script:StarshipExecutable = ::STARSHIP::
    $script:StreamTimings = ''
    $script:TransientPrompt = $false

    # The one shared cell the OnIdle pump reads; it always holds the live
    # stream (or $null). It must be a global: event actions run in a session
    # state of their own where module variables, closures, and even the
    # subscription's MessageData (PSReadLine generates OnIdle events with no
    # payload) all fail to resolve — a global is the only channel that works.
    $global:__StarshipStreamBox = @{ Stream = $null }
    $script:PumpJob = $null
    $script:PumpSubscriptionId = $null

    # Hosts without PSReadLine (the ISE, constrained hosts) still get a working
    # synchronous prompt; only streaming and transience require PSReadLine.
    $script:UsesPSReadLine = $null -ne (
        Get-Command -Name Get-PSReadLineOption -ErrorAction SilentlyContinue
    )
    $script:DoesUseLists = $script:UsesPSReadLine -and
        (Get-PSReadLineOption).PredictionViewStyle -eq 'ListView'

    function Get-Cwd {
        $cwd = Get-Location
        $providerPrefix = "$($cwd.Provider.ModuleName)\$($cwd.Provider.Name)::"
        @{
            # ProviderPath is physical only for the FileSystem provider.
            Path = $cwd.ProviderPath
            # Microsoft.PowerShell.Core\FileSystem::Dev:\ -> Dev:\
            LogicalPath = if ($cwd.Path.StartsWith($providerPrefix)) {
                $cwd.Path.Substring($providerPrefix.Length)
            } else {
                $cwd.Path
            }
        }
    }

    function New-NativeProcess {
        param(
            [Parameter(Mandatory)] [string] $Executable,
            [Parameter(Mandatory)] [string[]] $Arguments
        )

        $startInfo = New-Object System.Diagnostics.ProcessStartInfo -ArgumentList $Executable -Property @{
            StandardOutputEncoding = [System.Text.Encoding]::UTF8
            RedirectStandardOutput = $true
            RedirectStandardError = $true
            CreateNoWindow = $true
            UseShellExecute = $false
        }

        # ArgumentList bypasses Windows command-line re-parsing on modern .NET.
        # Windows PowerShell's .NET Framework lacks it, so retain the exact CRT
        # quoting algorithm as the compatibility path.
        if ($null -ne $startInfo.PSObject.Properties['ArgumentList']) {
            foreach ($argument in $Arguments) {
                $startInfo.ArgumentList.Add($argument)
            }
        } else {
            $escaped = $Arguments | ForEach-Object {
                $argument = $_ -Replace '(\\+)"', '$1$1"'
                $argument = $argument -Replace '(\\+)$', '$1$1'
                $argument = $argument -Replace '"', '\"'
                "`"$argument`""
            }
            $startInfo.Arguments = $escaped -Join ' '
        }

        [System.Diagnostics.Process]::Start($startInfo)
    }

    function Invoke-Native {
        param(
            [Parameter(Mandatory)] [string] $Executable,
            [Parameter(Mandatory)] [string[]] $Arguments
        )

        $process = New-NativeProcess -Executable $Executable -Arguments $Arguments
        try {
            # Drain both redirected pipes concurrently to rule out pipe-buffer
            # deadlocks, including on the synchronous fallback path.
            $stdout = $process.StandardOutput.ReadToEndAsync()
            $stderr = $process.StandardError.ReadToEndAsync()
            [System.Threading.Tasks.Task]::WaitAll(@($stdout, $stderr))

            if ($stderr.Result.Trim() -ne '') {
                $Host.UI.WriteErrorLine($stderr.Result)
            }
            $stdout.Result
        } finally {
            $process.Dispose()
        }
    }

    function Stop-StarshipStream {
        $stream = $global:__StarshipStreamBox.Stream
        if ($null -eq $stream) {
            return
        }
        $global:__StarshipStreamBox.Stream = $null
        if ($stream.Timings -ne '') {
            $script:StreamTimings = $stream.Timings
        }
        try {
            if (-not $stream.Process.HasExited) {
                $stream.Process.Kill()
            }
        } catch {}
        $stream.Process.Dispose()
    }

    function Start-StarshipStream {
        param([Parameter(Mandatory)] [string[]] $Arguments)

        $streamArguments = @('stream', '--frames=json') + $Arguments
        if ($script:StreamTimings -ne '') {
            $streamArguments += "--timings=$($script:StreamTimings)"
        }

        $process = $null
        try {
            $process = New-NativeProcess -Executable $script:StarshipExecutable -Arguments $streamArguments
            # Never surface renderer diagnostics into an editable command line,
            # and never let the renderer block on a full stderr pipe.
            $null = $process.StandardError.ReadToEndAsync()

            # The READY latch is a plain task wait: the reading happens on the
            # thread pool, so no PowerShell event pumping is involved at all.
            $reader = $process.StandardOutput
            $firstLine = $reader.ReadLineAsync()
            if (-not $firstLine.Wait(2000) -or $null -eq $firstLine.Result) {
                throw 'starship stream did not paint in time'
            }
            $frame = $firstLine.Result | ConvertFrom-Json -ErrorAction Stop
            if ($frame.kind -ne 'READY') {
                throw "starship stream began with $($frame.kind) instead of READY"
            }

            $global:__StarshipStreamBox.Stream = @{
                NextLine = $reader.ReadLineAsync()
                # What the terminal currently shows; the pump repaints
                # relative to it.
                Painted = [string]$frame.prompt
                Process = $process
                Prompt = [string]$frame.prompt
                Reader = $reader
                Timings = ''
            }
            $global:__StarshipStreamBox.Stream.Prompt
        } catch {
            if ($null -ne $process) {
                try {
                    if (-not $process.HasExited) {
                        $process.Kill()
                    }
                } catch {}
                $process.Dispose()
            }
            $null
        }
    }

    function Get-StarshipArguments {
        param(
            [Parameter(Mandatory)] [bool] $DollarQuestion,
            [Parameter(Mandatory)] [int] $LastExitCode
        )

        $cwd = Get-Cwd
        # The pump subscription is implementation detail, never a user job.
        $runningJobs = @(
            Get-Job | Where-Object {
                $_.State -eq 'Running' -and
                ($null -eq $script:PumpJob -or $_.Id -ne $script:PumpJob.Id)
            }
        ).Count
        $arguments = @(
            "--path=$($cwd.Path)"
            "--logical-path=$($cwd.LogicalPath)"
            "--terminal-width=$($Host.UI.RawUI.WindowSize.Width)"
            "--jobs=$runningJobs"
        )

        # A fresh console has no history and is considered successful.
        $status = 0
        if ($lastCommand = Get-History -Count 1) {
            if (-not $DollarQuestion) {
                $lastCmdletError = try {
                    $global:error[0] |
                        Where-Object { $null -ne $_ } |
                        Select-Object -ExpandProperty InvocationInfo
                } catch { $null }

                $status = if (
                    $null -ne $lastCmdletError -and
                    $lastCommand.CommandLine -eq $lastCmdletError.Line
                ) { 1 } else { $LastExitCode }
            }
            $duration = [math]::Round(
                ($lastCommand.EndExecutionTime - $lastCommand.StartExecutionTime).TotalMilliseconds
            )
            $arguments += "--cmd-duration=$duration"
        }

        $arguments += "--status=$status"
        if ($script:UsesPSReadLine -and [Microsoft.PowerShell.PSConsoleReadLine]::InViCommandMode()) {
            $arguments += '--keymap=vi'
        }
        $arguments
    }

    function Invoke-StarshipRender {
        param([Parameter(Mandatory)] [string[]] $Arguments)
        Invoke-Native -Executable $script:StarshipExecutable -Arguments (@('prompt') + $Arguments)
    }

    function Set-StarshipExtraPromptLineCount {
        param([Parameter(Mandatory)] [string] $PromptText)
        if ($script:UsesPSReadLine) {
            Set-PSReadLineOption -ExtraPromptLineCount ($PromptText.Split("`n").Length - 1)
        }
    }

    function Enable-TransientPrompt {
        if (-not $script:UsesPSReadLine) {
            return
        }
        Set-PSReadLineKeyHandler -Key Enter -ScriptBlock {
            $previousOutputEncoding = [Console]::OutputEncoding
            try {
                $parseErrors = $null
                [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState(
                    [ref]$null, [ref]$null, [ref]$parseErrors, [ref]$null
                )
                if ($parseErrors.Count -eq 0) {
                    Stop-StarshipStream
                    $script:TransientPrompt = $true
                    [Console]::OutputEncoding = [Text.Encoding]::UTF8
                    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
                }
            } finally {
                if ($script:DoesUseLists) {
                    # Clear the prompt, input, and at most ten prediction rows.
                    $rows = [math]::Min(
                        $Host.UI.RawUI.WindowSize.Height - $Host.UI.RawUI.CursorPosition.Y - 1,
                        12
                    )
                    [Microsoft.PowerShell.PSConsoleReadLine]::Insert("`n" * $rows)
                    [Microsoft.PowerShell.PSConsoleReadLine]::Undo()
                }
                [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
                [Console]::OutputEncoding = $previousOutputEncoding
            }
        }
    }

    function Disable-TransientPrompt {
        if ($script:UsesPSReadLine) {
            Set-PSReadLineKeyHandler -Key Enter -Function AcceptLine
        }
        $script:TransientPrompt = $false
    }

    function global:prompt {
        $origDollarQuestion = $global:?
        $origLastExitCode = $global:LASTEXITCODE

        Stop-StarshipStream

        try {
            if (Test-Path function:Invoke-Starship-PreCommand) {
                Invoke-Starship-PreCommand
            }
        } catch {}

        $argumentParameters = @{
            DollarQuestion = $origDollarQuestion
            LastExitCode = $origLastExitCode
        }
        $arguments = Get-StarshipArguments @argumentParameters

        $promptText = if ($script:TransientPrompt) {
            $script:TransientPrompt = $false
            if (Test-Path function:Invoke-Starship-TransientFunction) {
                Invoke-Starship-TransientFunction
            } else {
                "$([char]0x1B)[1;32m❯$([char]0x1B)[0m "
            }
        } else {
            # Streaming repaints ride on PSReadLine's idle pump, so without
            # PSReadLine the prompt simply renders synchronously.
            $streamed = if ($script:UsesPSReadLine) {
                Start-StarshipStream -Arguments $arguments
            } else {
                $null
            }
            if ($null -ne $streamed) {
                $streamed
            } else {
                Invoke-StarshipRender -Arguments $arguments
            }
        }

        Set-StarshipExtraPromptLineCount -PromptText $promptText
        $promptText

        # Preserve the status values from before prompt/event machinery ran.
        $global:LASTEXITCODE = $origLastExitCode
        if ($global:? -ne $origDollarQuestion) {
            if ($origDollarQuestion) {
                1 + 1
            } else {
                Write-Error '' -ErrorAction Ignore
            }
        }
    }

    # The entire live-update mechanism: PSReadLine wakes the engine every 300ms
    # while it waits for keys, but only when a PowerShell.OnIdle subscriber
    # exists — so this single permanent subscription both causes the wake-ups
    # and applies whatever frames have arrived. PSReadLine never fires OnIdle
    # while a command runs or while typed text sits in the buffer, which is
    # exactly when a repaint would be unwelcome, so no stop-at-Enter hook and
    # no per-stream registration or teardown are needed. The action is fully
    # self-contained (globals, .NET, and built-ins only) because it executes
    # in a session state where nothing from this module resolves.
    if ($script:UsesPSReadLine) {
        $script:PumpJob = Register-EngineEvent -SourceIdentifier PowerShell.OnIdle -Action {
            $stream = $global:__StarshipStreamBox.Stream
            if ($null -eq $stream) {
                return
            }

            # Apply every frame the renderer has finished writing. Never
            # blocks: only completed read tasks are consumed.
            $changed = $false
            while ($null -ne $stream.NextLine -and $stream.NextLine.IsCompleted) {
                $line = $null
                try { $line = $stream.NextLine.Result } catch {}
                if ($null -eq $line) {
                    # End of stream. Normal after COMPLETE; after a renderer
                    # crash the last good paint stays until the next prompt.
                    $stream.NextLine = $null
                    break
                }
                $stream.NextLine = $stream.Reader.ReadLineAsync()

                try {
                    $frame = $line | ConvertFrom-Json -ErrorAction Stop
                } catch {
                    continue
                }
                switch ($frame.kind) {
                    'PATCH' {
                        $stream.Prompt = [string]$frame.prompt
                        $changed = $true
                    }
                    'COMPLETE' {
                        $stream.Timings = $frame.timings | ConvertTo-Json -Compress
                    }
                }
            }

            if (-not $changed) {
                return
            }

            # Repaint only the prompt lines above the input line, with purely
            # relative cursor movement. Two hard-won constraints shape this:
            #
            #   * On Unix, any write through the .NET console layer (Console,
            #     $Host.UI, even the raw stdout stream) while PSReadLine waits
            #     for keys silently stops its idle pump for the rest of the
            #     input session — measured, not theoretical. Writing to the
            #     terminal device directly leaves it running. Windows consoles
            #     have no such failure mode, so the host writer is fine there.
            #
            #   * The final prompt line is PSReadLine's anchor for where typed
            #     input begins, so it is never redrawn and the line count is
            #     never changed; PSReadLine's model of the screen stays valid
            #     without telling it anything. A patch that would change the
            #     line structure or the final line is skipped — the next
            #     prompt renders it anyway.
            $oldLines = $stream.Painted.Split("`n")
            $newLines = $stream.Prompt.Split("`n")
            if (
                $newLines.Length -le 1 -or
                $newLines.Length -ne $oldLines.Length -or
                $newLines[-1] -cne $oldLines[-1]
            ) {
                return
            }

            $escape = [char]27
            $sequence = New-Object System.Text.StringBuilder
            # Hide the cursor, save its position, and hop to the first line.
            $null = $sequence.Append("$escape[?25l$escape").Append('7')
            $null = $sequence.Append("`r$escape[").Append($newLines.Length - 1).Append('A')
            for ($index = 0; $index -lt $newLines.Length - 1; $index++) {
                $null = $sequence.Append($newLines[$index]).Append("$escape[K")
                if ($index -lt $newLines.Length - 2) {
                    $null = $sequence.Append("`r`n")
                }
            }
            $null = $sequence.Append("$escape").Append('8').Append("$escape[?25h")

            try {
                if ([System.Environment]::OSVersion.Platform -eq 'Win32NT') {
                    $Host.UI.Write($sequence.ToString())
                } else {
                    $terminal = [IO.File]::OpenWrite('/dev/tty')
                    try {
                        $bytes = [Text.Encoding]::UTF8.GetBytes($sequence.ToString())
                        $terminal.Write($bytes, 0, $bytes.Length)
                    } finally {
                        $terminal.Dispose()
                    }
                }
                $stream.Painted = $stream.Prompt
            } catch {}
        }
        $script:PumpSubscriptionId = (
            Get-EventSubscriber |
                Where-Object { $_.Action -eq $script:PumpJob } |
                Select-Object -First 1
        ).SubscriptionId
    }

    $ENV:VIRTUAL_ENV_DISABLE_PROMPT = 1
    $ENV:STARSHIP_SHELL = if ($PSVersionTable.PSVersion.Major -gt 5) { 'pwsh' } else { 'powershell' }
    $ENV:STARSHIP_SESSION_KEY = -join (
        (48..57) + (65..90) + (97..122) |
            Get-Random -Count 16 |
            ForEach-Object { [char]$_ }
    )

    if ($script:UsesPSReadLine) {
        Set-PSReadLineOption -ContinuationPrompt (
            Invoke-Native -Executable $script:StarshipExecutable -Arguments @('prompt', '--continuation')
        )

        try {
            $viModeHandler = (Get-PSReadLineOption).ViModeChangeHandler
            if ($viModeHandler) {
                Set-PSReadLineOption -ViModeChangeHandler {
                    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
                    & $viModeHandler @args
                }.GetNewClosure()
            } else {
                Set-PSReadLineOption -ViModeIndicator Script -ViModeChangeHandler {
                    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
                }
            }
        } catch {}
    }

    $ExecutionContext.SessionState.Module.OnRemove = {
        Stop-StarshipStream
        if ($null -ne $script:PumpSubscriptionId) {
            Unregister-Event -SubscriptionId $script:PumpSubscriptionId -ErrorAction SilentlyContinue
        }
        if ($null -ne $script:PumpJob) {
            Remove-Job -Job $script:PumpJob -Force -ErrorAction SilentlyContinue
        }
        Remove-Variable -Name __StarshipStreamBox -Scope Global -ErrorAction SilentlyContinue
    }

    Export-ModuleMember -Function @(
        'Enable-TransientPrompt'
        'Disable-TransientPrompt'
    )
}
