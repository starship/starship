#!/usr/bin/env pwsh

# Replace an earlier initialization cleanly. In particular, this runs the old
# module's OnRemove hook so no renderer process or pump pipeline survives a reload.
Get-Module -Name starship | Remove-Module -Force

# Keep Starship's implementation out of the global namespace. Only the prompt
# function and the two public transience controls escape this dynamic module.
$null = New-Module starship {
    $script:StarshipExecutable = ::STARSHIP::
    $script:StreamTimings = ''
    $script:TransientPrompt = $false

    # The box is the only live-stream state. It is touched by the prompt thread
    # and the pump concurrently, hence synchronized.
    $script:Stream = $null

    # Hosts without PSReadLine (the ISE, constrained hosts) still get a working
    # synchronous prompt; only streaming and transience require PSReadLine.
    $script:UsesPSReadLine = $null -ne (
        Get-Command -Name Get-PSReadLineOption -ErrorAction SilentlyContinue
    )
    $script:DoesUseLists = $script:UsesPSReadLine -and
        (Get-PSReadLineOption).PredictionViewStyle -eq 'ListView'

    # Start-StarshipStream copies these module-private functions into the
    # private runspace and invokes the pump by command name.
    function Write-StarshipStreamTty {
        param([Parameter(Mandatory)] [string] $Text)

        $bytes = [System.Text.Encoding]::UTF8.GetBytes($Text)
        $tty = if ($env:OS -eq 'Windows_NT') {
            [Console]::OpenStandardOutput()
        } else {
            [System.IO.File]::Open(
                '/dev/tty',
                [System.IO.FileMode]::Open,
                [System.IO.FileAccess]::Write
            )
        }
        try {
            $tty.Write($bytes, 0, $bytes.Length)
            $tty.Flush()
        } finally {
            $tty.Dispose()
        }
    }

    function Stop-StarshipProcess {
        param([System.Diagnostics.Process] $Process)

        if ($Process) {
            try { $Process.Kill() } catch { }
            $Process.Dispose()
        }
    }

    function Invoke-StarshipStreamPump {
        param(
            [System.Diagnostics.ProcessStartInfo] $StartInfo,
            [hashtable] $Box
        )

        $process = $null
        try {
            $process = [System.Diagnostics.Process]::Start($StartInfo)
            $Box.Process = $process
            $null = $process.StandardError.ReadToEndAsync()
            $process.StandardInput.Close()
            $reader = $process.StandardOutput

            $frame = ConvertFrom-Json -InputObject $reader.ReadLine() -ErrorAction Stop
            if ($frame.kind -ne 'READY') {
                throw "starship stream began with '$($frame.kind)' instead of READY"
            }
            $Box.Prompt = $frame.prompt
            $null = $Box.Ready.Set()

            while (-not $Box.Stop) {
                $line = $reader.ReadLine()
                if ($null -eq $line) { break }
                $frame = ConvertFrom-Json -InputObject $line -ErrorAction Stop
                switch ($frame.kind) {
                    'PATCH' {
                        $Box.Prompt = $frame.prompt
                        if ($null -eq $frame.repaint) {
                            $Box.Stop = $true
                        } else {
                            $commandLine = $null; $cursor = 0
                            $null = [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState(
                                [ref]$commandLine, [ref]$cursor
                            )
                            if (-not $commandLine) {
                                Write-StarshipStreamTty $frame.repaint
                            }
                        }
                    }
                    'COMPLETE' {
                        $Box.Timings = ConvertTo-Json $frame.timings -Compress
                    }
                }
            }
        } catch {
            if (-not $Box.Stop) {
                $null = $Box.Ready.Set()
                [Console]::Error.WriteLine("starship: the stream's pump stopped: $_")
            }
        } finally {
            Stop-StarshipProcess $process
        }
    }

    # Reuse one initial-state template; each runspace gets its own execution
    # state while the function definitions are captured only once per module.
    $script:StreamState = [System.Management.Automation.Runspaces.InitialSessionState]::CreateDefault()
    Get-Command Write-StarshipStreamTty, Invoke-StarshipStreamPump, Stop-StarshipProcess -CommandType Function |
        ForEach-Object {
            $null = $script:StreamState.Commands.Add(
                [System.Management.Automation.Runspaces.SessionStateFunctionEntry]::new(
                    $_.Name, $_.Definition
                )
            )
        }

    function New-StarshipStartInfo {
        param([Parameter(Mandatory)] [string[]] $Arguments)

        $startInfo = [System.Diagnostics.ProcessStartInfo]::new($script:StarshipExecutable)
        $startInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8
        $startInfo.RedirectStandardOutput =
            $startInfo.RedirectStandardError =
            $startInfo.RedirectStandardInput = $true
        $startInfo.CreateNoWindow = $startInfo.UseShellExecute = $false

        # ArgumentList bypasses Windows command-line re-parsing on modern .NET.
        # Windows PowerShell's .NET Framework lacks it, so retain the exact CRT
        # quoting algorithm as the compatibility path.
        if ($null -ne $startInfo.PSObject.Properties['ArgumentList']) {
            foreach ($argument in $Arguments) {
                $null = $startInfo.ArgumentList.Add($argument)
            }
        } else {
            $escaped = $Arguments -replace '(\\+)"', '$1$1"' -replace '(\\+)$', '$1$1' -replace '"', '\"'
            $startInfo.Arguments = ($escaped | ForEach-Object { "`"$_`"" }) -join ' '
        }
        $startInfo
    }

    function Invoke-Starship {
        param([Parameter(Mandatory)] [string[]] $Arguments)

        $process = [System.Diagnostics.Process]::Start((New-StarshipStartInfo $Arguments))
        # Closed immediately: stdin is redirected only to keep the interactive
        # terminal off the child's hands, and closing it hands back an EOF.
        $process.StandardInput.Close()
        try {
            # Drain both pipes concurrently to rule out buffer-full deadlocks.
            $stdout = $process.StandardOutput.ReadToEndAsync()
            $stderr = $process.StandardError.ReadToEndAsync()
            [System.Threading.Tasks.Task]::WaitAll($stdout, $stderr)
            if ($stderr.Result.Trim()) {
                $Host.UI.WriteErrorLine($stderr.Result)
            }
            $stdout.Result
        } finally {
            $process.Dispose()
        }
    }

    function Stop-StarshipStream {
        $box = $script:Stream
        $script:Stream = $null
        if ($null -eq $box) { return }

        if ($box.Timings) {
            $script:StreamTimings = $box.Timings
        }
        # Kill the renderer to close its output pipe, which unblocks the pump's
        # next read. The engine join is bounded because shutdown can race start.
        $box.Stop = $true
        try { Stop-StarshipProcess $box.Process } catch { }
        try {
            if ($box.Engine) {
                $null = $box.Engine.BeginStop($null, $null).AsyncWaitHandle.WaitOne(1000)
                $box.Engine.Runspace.Dispose()
                $box.Engine.Dispose()
            }
        } catch { }
        $box.Ready.Dispose()
    }

    function Start-StarshipStream {
        param([Parameter(Mandatory)] [string[]] $Arguments)

        $box = [System.Collections.Hashtable]::Synchronized(@{
            Ready = [System.Threading.ManualResetEventSlim]::new()
            Stop = $false
            Timings = ''
            Process = $null
            Prompt = $null
            Engine = $null
        })

        $arguments = @('stream', '--frames', 'json') + $Arguments
        if ($script:StreamTimings) {
            $arguments += "--timings=$($script:StreamTimings)"
        }

        try {
            $script:Stream = $box
            $box.Engine = [powershell]::Create()
            $box.Engine.Runspace = [System.Management.Automation.Runspaces.RunspaceFactory]::CreateRunspace($script:StreamState)
            $box.Engine.Runspace.Open()
            $null = $box.Engine.AddCommand('Invoke-StarshipStreamPump').
                AddArgument((New-StarshipStartInfo $arguments)).
                AddArgument($box)
            $null = $box.Engine.BeginInvoke()

            # The handshake bound: READY within two seconds, or tear the stream
            # down and let the caller fall back to a synchronous render — the
            # same contract the old native Attach enforced with its own timeouts.
            if ($box.Ready.Wait(2000) -and $null -ne $box.Prompt) {
                return $box.Prompt
            }
        } catch {
            $box.Stop = $true
        }
        Stop-StarshipStream
    }

    function Get-StarshipArguments {
        param(
            [Parameter(Mandatory)] [bool] $DollarQuestion,
            [Parameter(Mandatory)] [int] $LastExitCode
        )

        $location = Get-Location
        # ProviderPath is physical only for the FileSystem provider, whose
        # prefix is what turns a logical Path into a physical one.
        $prefix = "$($location.Provider.ModuleName)\$($location.Provider.Name)::"
        $logicalPath = if ($location.Path.StartsWith($prefix)) {
            $location.Path.Substring($prefix.Length)
        } else {
            $location.Path
        }

        $arguments = @(
            "--path=$($location.ProviderPath)"
            "--logical-path=$logicalPath"
            "--terminal-width=$($Host.UI.RawUI.WindowSize.Width)"
            "--jobs=$(@(
                Get-Job |
                    Where-Object State -EQ Running
            ).Count)"
        )

        # A fresh console has no history and is considered successful. When the
        # last command failed, the failure belongs to this command line only if
        # the newest error record was raised by it; otherwise a native exit
        # code is the honest status.
        $status = 0
        if ($lastCommand = Get-History -Count 1) {
            if (-not $DollarQuestion) {
                $lastCmdletError = $global:error[0].InvocationInfo
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

    function Set-StarshipExtraPromptLineCount {
        param([Parameter(Mandatory)] [string] $PromptText)
        if ($script:UsesPSReadLine) {
            Set-PSReadLineOption -ExtraPromptLineCount ($PromptText.Split("`n").Count - 1)
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
        } catch { }

        $arguments = Get-StarshipArguments -DollarQuestion $origDollarQuestion -LastExitCode $origLastExitCode

        $promptText = if ($script:TransientPrompt) {
            $script:TransientPrompt = $false
            if (Test-Path function:Invoke-Starship-TransientFunction) {
                Invoke-Starship-TransientFunction
            } else {
                "$([char]27)[1;32m❯$([char]27)[0m "
            }
        } elseif (
            $script:UsesPSReadLine -and
            $null -ne ($streamed = Start-StarshipStream $arguments)
        ) {
            # Set-StarshipExtraPromptLineCount needs PSReadLine to act on it,
            # so without PSReadLine the prompt renders synchronously rather
            # than starting a stream nothing would ever redraw.
            $streamed
        } else {
            Invoke-Starship (@('prompt') + $arguments)
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

    $ENV:VIRTUAL_ENV_DISABLE_PROMPT = 1
    $ENV:STARSHIP_SHELL = ('powershell', 'pwsh')[$PSVersionTable.PSVersion.Major -gt 5]
    $ENV:STARSHIP_SESSION_KEY = [guid]::NewGuid().ToString('N').Substring(0, 16)

    if ($script:UsesPSReadLine) {
        Set-PSReadLineOption -ContinuationPrompt (
            Invoke-Starship @('prompt', '--continuation')
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
        } catch { }
    }

    $ExecutionContext.SessionState.Module.OnRemove = {
        Stop-StarshipStream
    }

    Export-ModuleMember -Function @(
        'Enable-TransientPrompt'
        'Disable-TransientPrompt'
    )
}
