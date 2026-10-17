#!/usr/bin/env pwsh

# Create a new dynamic module so we don't pollute the global namespace with our functions and
# variables
$null = New-Module starship {
    function Get-Cwd {
        $cwd = Get-Location
        $provider_prefix = "$($cwd.Provider.ModuleName)\$($cwd.Provider.Name)::"
        return @{
            # Resolve the actual/physical path
            # NOTE: ProviderPath is only a physical filesystem path for the "FileSystem" provider
            # E.g. `Dev:\` -> `C:\Users\Joe Bloggs\Dev\`
            Path = $cwd.ProviderPath;
            # Resolve the provider-logical path
            # NOTE: Attempt to trim any "provider prefix" from the path string.
            # E.g. `Microsoft.PowerShell.Core\FileSystem::Dev:\` -> `Dev:\`
            LogicalPath =
                if ($cwd.Path.StartsWith($provider_prefix)) {
                    $cwd.Path.Substring($provider_prefix.Length)
                } else {
                    $cwd.Path
                };
        }
    }

    function Start-Native {
        param($Executable, $Arguments)
        $startInfo = New-Object System.Diagnostics.ProcessStartInfo -ArgumentList $Executable -Property @{
            StandardOutputEncoding = [System.Text.Encoding]::UTF8;
            RedirectStandardOutput = $true;
            RedirectStandardError = $true;
            CreateNoWindow = $true;
            UseShellExecute = $false;
        };
        if ($startInfo.ArgumentList.Add) {
            # PowerShell 6+ uses .NET 5+ and supports the ArgumentList property
            # which bypasses the need for manually escaping the argument list into
            # a command string.
            foreach ($arg in $Arguments) {
                $startInfo.ArgumentList.Add($arg);
            }
        }
        else {
            # Build an arguments string which follows the C++ command-line argument quoting rules
            # See: https://docs.microsoft.com/en-us/previous-versions//17w5ykft(v=vs.85)?redirectedfrom=MSDN
            $escaped = $Arguments | ForEach-Object {
                $s = $_ -Replace '(\\+)"','$1$1"'; # Escape backslash chains immediately preceding quote marks.
                $s = $s -Replace '(\\+)$','$1$1';  # Escape backslash chains immediately preceding the end of the string.
                $s = $s -Replace '"','\"';         # Escape quote marks.
                "`"$s`""                           # Quote the argument.
            }
            $startInfo.Arguments = $escaped -Join ' ';
        }
        [System.Diagnostics.Process]::Start($startInfo)
    }

    function Invoke-Native {
        param($Executable, $Arguments)
        $process = Start-Native $Executable $Arguments

        # Read the output and error streams asynchronously
        # Avoids potential deadlocks when the child process fills one of the buffers
        # https://docs.microsoft.com/en-us/dotnet/api/system.diagnostics.process.standardoutput?view=net-6.0#remarks
        $stdout = $process.StandardOutput.ReadToEndAsync()
        $stderr = $process.StandardError.ReadToEndAsync()
        [System.Threading.Tasks.Task]::WaitAll(@($stdout, $stderr))

        # stderr isn't displayed with this style of invocation
        # Manually write it to console
        if ($stderr.Result.Trim() -ne '') {
            # Write-Error doesn't work here
            $host.ui.WriteErrorLine($stderr.Result)
        }

        $stdout.Result;
    }

    # PSReadLine redraws a prompt once while it reads a line: the stream draws
    # the prompt, draws it again once complete if that changes it, and ends.
    $script:StreamPump = {
        param($Stream, $Events)
        $block = [byte[]]::new(65536)
        $characters = [char[]]::new([Text.Encoding]::UTF8.GetMaxCharCount($block.Length))
        $decoder = [Text.Encoding]::UTF8.GetDecoder()
        $received = ''
        $fields = @()
        try {
            while (($read = $Stream.Process.StandardOutput.BaseStream.ReadAsync(
                        $block, 0, $block.Length, $Stream.Cancellation.Token).GetAwaiter().GetResult()) -gt 0) {
                $received += [string]::new($characters, 0, $decoder.GetChars($block, 0, $read, $characters, 0))
                # PSReadLine has no right prompt; the first of each pair is the main one.
                $fields = $received.Split([char] 0)
                if ($fields.Count -ge 3 -and -not $Stream.Ready.IsSet) {
                    $Stream.Prompt = $fields[0]
                    $Stream.Ready.Set()
                }
            }
            if ($fields.Count -ge 5) {
                $Stream.Prompt = $fields[2]
                $null = $Events.GenerateEvent('Starship.Refined', $Stream, $null, $null)
            }
        } catch [System.OperationCanceledException] {
            if (-not $Stream.Cancellation.IsCancellationRequested) { throw }
        } finally {
            $Stream.Process.StandardOutput.Dispose()
            $Stream.Ready.Set()
        }
    }

    function Test-StarshipReadingLine {
        (Get-PSCallStack).Command -contains 'PSConsoleHostReadLine'
    }

    # Whether nothing has been typed into the line PSReadLine reads.
    function Test-StarshipLineEmpty {
        $line = $null
        $cursor = $null
        [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$line, [ref]$cursor)
        -not $line
    }

    # Ignore stale streams and accepted lines to preserve command output, and
    # lines being typed: PSReadLine misplaces a prompt redrawn beneath them.
    function Update-StarshipPrompt {
        param($Stream)
        if ([object]::ReferenceEquals($Stream, $script:Stream) -and (Test-StarshipReadingLine) -and (Test-StarshipLineEmpty)) {
            $script:Refining = $true
            try {
                [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
            } finally {
                $script:Refining = $false
            }
        }
    }

    function Start-StarshipStream {
        param($Arguments)
        $arguments = @("prompt", "--stream", "--redraws=1") + $Arguments
        $stream = [hashtable]::Synchronized(@{
            Ready = [System.Threading.ManualResetEventSlim]::new()
            Prompt = $null
            Cancellation = [System.Threading.CancellationTokenSource]::new()

            # Event actions run in another module; carry this module's redraw closure.
            Redraw = ${function:Update-StarshipPrompt}
        })

        if ($script:Unix) {
            # An exiting direct child pauses PSReadLine events until a keypress.
            # Launch as a grandchild through a short-lived shell instead.
            $stream.Process = Start-Native '/bin/sh' (
                @('-c', '"$0" "$@" 2>/dev/null &', ::STARSHIP::) + $arguments)
            $null = $stream.Process.StandardError.ReadToEnd()
            $stream.Process.WaitForExit()
        } else {
            $stream.Process = Start-Native ::STARSHIP:: $arguments
            # Drain the pipe so the writer can finish.
            $null = $stream.Process.StandardError.ReadToEndAsync()
        }
        $script:Pump.Commands.Clear()
        $null = $script:Pump.AddScript($script:StreamPump)
        $null = $script:Pump.AddArgument($stream).AddArgument($ExecutionContext.Events)
        $stream.Pumping = $script:Pump.BeginInvoke()
        $script:Stream = $stream
        $stream.Ready.Wait()
        if ($null -ne $stream.Prompt) {
            $stream.Prompt
        } else {
            Stop-StarshipStream
            $null
        }
    }

    function Stop-StarshipStream {
        $stream = $script:Stream
        if ($null -eq $stream) { return }
        $script:Stream = $null
        # A stream ends once nothing reads it. Framework's reads cannot be
        # cancelled, so on Windows it is ended by the handle to its process,
        # never by a process ID that may have been reused.
        try {
            $stream.Cancellation.Cancel()
            if (-not $script:Unix) {
                try { $stream.Process.Kill() } catch [InvalidOperationException] {}
            }
            $null = $script:Pump.EndInvoke($stream.Pumping)
        } finally {
            $stream.Cancellation.Dispose()
            $stream.Ready.Dispose()
            $stream.Process.Dispose()
        }
    }

    function Enable-TransientPrompt {
        Set-PSReadLineKeyHandler -Key Enter -ScriptBlock {
            $previousOutputEncoding = [Console]::OutputEncoding
            try {
                $parseErrors = $null
                [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$null, [ref]$null, [ref]$parseErrors, [ref]$null)
                if ($parseErrors.Count -eq 0) {
                    $script:TransientPrompt = $true
                    [Console]::OutputEncoding = [Text.Encoding]::UTF8
                    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
                }
            } finally {
                if ($script:DoesUseLists) {
                    # If PSReadline is set to display suggestion list, this workaround is needed to clear the buffer below
                    # before accepting the current commandline. The max amount of items in the list is 10, so 12 lines
                    # are cleared (10 + 1 more for the prompt + 1 more for current commandline).
                    [Microsoft.PowerShell.PSConsoleReadLine]::Insert("`n" * [math]::Min($Host.UI.RawUI.WindowSize.Height - $Host.UI.RawUI.CursorPosition.Y - 1, 12))
                    [Microsoft.PowerShell.PSConsoleReadLine]::Undo()
                }
                [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
                [Console]::OutputEncoding = $previousOutputEncoding
            }
        }
    }

    function Disable-TransientPrompt {
        Set-PSReadLineKeyHandler -Key Enter -Function AcceptLine
        $script:TransientPrompt = $false
    }

    function global:prompt {
        $origDollarQuestion = $global:?
        $origLastExitCode = $global:LASTEXITCODE

        # Invoke precmd, if specified
        try {
            if (Test-Path function:Invoke-Starship-PreCommand) {
                Invoke-Starship-PreCommand
            }
        } catch {}

        # @ makes sure the result is an array even if single or no values are returned
        $jobs = @(Get-Job | Where-Object { $_.State -eq 'Running' }).Count

        $cwd = Get-Cwd
        $arguments = @(
            "--path=$($cwd.Path)",
            "--logical-path=$($cwd.LogicalPath)",
            "--terminal-width=$($Host.UI.RawUI.WindowSize.Width)",
            "--jobs=$($jobs)"
        )

        # We start from the premise that the command executed correctly, which covers also the fresh console.
        $lastExitCodeForPrompt = 0
        if ($lastCmd = Get-History -Count 1) {
            # In case we have a False on the Dollar hook, we know there's an error.
            if (-not $origDollarQuestion) {
                # We retrieve the InvocationInfo from the most recent error using $global:error[0]
                $lastCmdletError = try { $global:error[0] |  Where-Object { $_ -ne $null } | Select-Object -ExpandProperty InvocationInfo } catch { $null }
                # We check if the last command executed matches the line that caused the last error, in which case we know
                # it was an internal Powershell command, otherwise, there MUST be an error code.
                $lastExitCodeForPrompt = if ($null -ne $lastCmdletError -and $lastCmd.CommandLine -eq $lastCmdletError.Line) { 1 } else { $origLastExitCode }
            }
            $duration = [math]::Round(($lastCmd.EndExecutionTime - $lastCmd.StartExecutionTime).TotalMilliseconds)

            $arguments += "--cmd-duration=$($duration)"
        }

        $arguments += "--status=$($lastExitCodeForPrompt)"

        if ([Microsoft.PowerShell.PSConsoleReadLine]::InViCommandMode()) {
            $arguments += "--keymap=vi"
        }

        # Invoke Starship
        $promptText = if ($script:Refining) {
            $script:Stream.Prompt
        } else {
            Stop-StarshipStream
            if ($script:TransientPrompt) {
                $script:TransientPrompt = $false
                if (Test-Path function:Invoke-Starship-TransientFunction) {
                    Invoke-Starship-TransientFunction
                } else {
                    "$([char]0x1B)[1;32m❯$([char]0x1B)[0m "
                }
            } else {
                # Draw vi mode changes in full; this line may have used its redraw.
                $streamed = if ($script:Pump -and -not (Test-StarshipReadingLine)) {
                    Start-StarshipStream $arguments
                }
                if ($null -ne $streamed) {
                    $streamed
                } else {
                    Invoke-Native -Executable ::STARSHIP:: -Arguments (@("prompt") + $arguments)
                }
            }
        }

        # Set the number of extra lines in the prompt for PSReadLine prompt redraw.
        Set-PSReadLineOption -ExtraPromptLineCount ($promptText.Split("`n").Length - 1)

        # Return the prompt
        $promptText

        # Propagate the original $LASTEXITCODE from before the prompt function was invoked.
        $global:LASTEXITCODE = $origLastExitCode

        # Propagate the original $? automatic variable value from before the prompt function was invoked.
        #
        # $? is a read-only or constant variable so we can't directly override it.
        # In order to propagate up its original boolean value we will take an action
        # which will produce the desired value.
        #
        # This has to be the very last thing that happens in the prompt function
        # since every PowerShell command sets the $? variable.
        if ($global:? -ne $origDollarQuestion) {
            if ($origDollarQuestion) {
                 # Simple command which will execute successfully and set $? = True without any other side affects.
                1+1
            } else {
                # Write-Error will set $? to False.
                # ErrorAction Ignore will prevent the error from being added to the $Error collection.
                Write-Error '' -ErrorAction 'Ignore'
            }
        }

    }

    # Disable virtualenv prompt, it breaks starship
    $ENV:VIRTUAL_ENV_DISABLE_PROMPT=1

    $script:TransientPrompt = $false
    $script:DoesUseLists = (Get-PSReadLineOption).PredictionViewStyle -eq 'ListView'

    $script:Stream = $null
    $script:Refining = $false
    $script:Pump = $null
    $script:Unix = $PSVersionTable.PSVersion.Major -gt 5 -and -not $IsWindows

    # Without PSReadLine, render the complete prompt.
    if (Get-Command PSConsoleHostReadLine -ErrorAction Ignore) {
        $script:Pump = [powershell]::Create()
        $script:Pump.Runspace = [runspacefactory]::CreateRunspace()
        $script:Pump.Runspace.Open()

        # PSReadLine ignores support events while waiting for input;
        # register a normal event for the completion redraw.
        $null = Register-EngineEvent -SourceIdentifier Starship.Refined -Action {
            $null = $Sender.Redraw.Invoke($Sender)
        }
    }

    if ($PSVersionTable.PSVersion.Major -gt 5) {
        $ENV:STARSHIP_SHELL = "pwsh"
    } else {
        $ENV:STARSHIP_SHELL = "powershell"
    }

    # Set up the session key that will be used to store logs
    $ENV:STARSHIP_SESSION_KEY = -join ((48..57) + (65..90) + (97..122) | Get-Random -Count 16 | ForEach-Object { [char]$_ })

    # Invoke Starship and set continuation prompt
    Set-PSReadLineOption -ContinuationPrompt (
        Invoke-Native -Executable ::STARSHIP:: -Arguments @(
            "prompt",
            "--continuation"
        )
    )

    try {
        # Combine user defined ViModeChangeHandler if it exists
        if((Get-PSReadLineOption).ViModeChangeHandler){
            # &{...} to limit the scope of the GetNewClosure
            & {
                $originalHandler = (Get-PSReadLineOption).ViModeChangeHandler
                Set-PSReadLineOption -ViModeChangeHandler {
                    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
                    & $originalHandler @args
                }.GetNewClosure()
            }
        } else {
            Set-PSReadLineOption -ViModeIndicator script -ViModeChangeHandler {
                [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
            }
        }
    } catch {}

    Export-ModuleMember -Function @(
        "Enable-TransientPrompt"
        "Disable-TransientPrompt"
    )
}
