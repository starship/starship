#!/usr/bin/env pwsh

# Re-`. $init` fires OnRemove, so no renderer or pump runspace survives a reload.
Get-Module starship | Remove-Module -Force

$null = New-Module starship {
    # Declared up front so a caller's Set-StrictMode never meets an unset name.
    $script:StreamTimings = ''
    $script:TransientPrompt = $false
    $script:StreamProcess = $script:PumpHandle = $script:Engine = $script:Terminal = $null

    # ISE and constrained hosts have no PSReadLine: they get the synchronous
    # prompt only, since streaming and transience both redraw through it. Naming
    # a cmdlet first is what auto-loads the module the type lives in.
    $script:LineEditor = if (Get-Command Set-PSReadLineOption -ErrorAction SilentlyContinue) {
        'Microsoft.PowerShell.PSConsoleReadLine' -as [type]
    }

    # Relays one renderer's frames until its pipe closes, on a private runspace
    # because nothing else can: PowerShell dispatches no event action while
    # PSReadLine owns the keyboard, and a thread job's runspace stalls behind
    # Console.ReadKey on Unix (PSReadLine#1092). The first paint goes back
    # through $Ready; the timings go down the pipeline, for EndInvoke to collect.
    function Invoke-StarshipPump {
        param([IO.TextReader] $Reader, [IO.TextWriter] $Terminal, $Ready)

        # A field ends at a NUL, the one byte a payload can never carry, so a
        # multi-line prompt comes back verbatim with nothing to unescape. Read
        # fills whatever the reader already has, so one call usually carries
        # several frames; what is left over stays for the next field to take.
        # A closed pipe reads as no characters, which ends the pump.
        #
        # IndexOf must take a [char]. The string overload compares with culture,
        # a NUL collates as nothing, and so it answers 0 for any text at all: the
        # blank line an `add_newline` prompt opens with would end the field there
        # and every prompt would arrive empty.
        # A [ref] cell, because a nested function that assigns to a captured
        # variable writes a local of its own instead.
        $buffer = [ref] ''
        $block = [char[]]::new(8192)
        function Read-Field {
            while (($cut = $buffer.Value.IndexOf([char] 0)) -lt 0) {
                $read = $Reader.Read($block, 0, $block.Length)
                if ($read -le 0) { return }
                $buffer.Value += [string]::new($block, 0, $read)
            }
            $field = $buffer.Value.Substring(0, $cut)
            $buffer.Value = $buffer.Value.Substring($cut + 1)
            $field
        }

        try {
            if ((Read-Field) -ne 'READY') { return }
            $Ready.SetResult((Read-Field))
            $null = Read-Field  # The renderer's process id; this shell holds the handle.
            while ($null -ne ($keyword = Read-Field)) {
                $first = Read-Field
                $second = Read-Field
                if ($keyword -eq 'COMPLETE') {
                    $first
                } elseif ($keyword -eq 'PATCH' -and $second) {
                    # Cell-precise bytes may only land on an empty input buffer;
                    # anything typed owns the line instead.
                    $line = $null
                    [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$line, [ref]$null)
                    if (-not $line) { $Terminal.Write($second) }
                }
            }
        } catch {
            # Losing the pipe mid-read is how a killed renderer ends, not a
            # fault to re-raise out of EndInvoke and into the next prompt.
        } finally { $null = $Ready.TrySetResult($null) }
    }

    # One runspace, one compiled pump and one autoflushing writer serve every
    # prompt of the session, so a stream costs a process and nothing else.
    # Repaints go to the console's own device, which no redirection of stdout
    # can take away, and unadorned: a BOM would print as a stray glyph.
    #
    # A runspace is a session state of its own, so the pump cannot be called
    # into it — it is defined there once, out of the very body compiled above.
    if ($script:LineEditor) {
        try {
            $device = if ($env:OS -eq 'Windows_NT') {
                [Console]::OpenStandardOutput()
            } else {
                [IO.File]::Open('/dev/tty', 'Open', 'Write', 'ReadWrite')
            }
            $script:Terminal = [IO.StreamWriter]::new($device, [Text.UTF8Encoding]::new($false))
            $script:Terminal.AutoFlush = $true
            $script:Engine = [powershell]::Create()
            $script:Engine.Runspace = [runspacefactory]::CreateRunspace()
            $script:Engine.Runspace.Open()
            $null = $script:Engine.AddScript(
                "function Invoke-StarshipPump { ${function:Invoke-StarshipPump} }").Invoke()
        } catch { $script:Engine = $null }
    }

    function Start-StarshipProcess {
        $info = [Diagnostics.ProcessStartInfo]::new(::STARSHIP::)
        $info.StandardOutputEncoding = [Text.Encoding]::UTF8
        $info.RedirectStandardOutput = $info.RedirectStandardError = $info.RedirectStandardInput = $true
        $info.UseShellExecute = $false
        $info.CreateNoWindow = $true
        # ArgumentList skips Windows CRT re-parse; Windows PowerShell 5 lacks it.
        if ($info.PSObject.Properties['ArgumentList']) {
            $args.ForEach{ $info.ArgumentList.Add($_) }
        } else {
            $info.Arguments = (($args -replace '(\\+)"', '$1$1"' -replace '(\\+)$', '$1$1' -replace '"', '\"').
                ForEach{ '"' + $_ + '"' }) -join ' '
        }
        $process = [Diagnostics.Process]::Start($info)
        # stdin is redirected only to keep the terminal out of the child's hands;
        # closing it hands back the EOF it expects. stderr has to be drained
        # while the child runs, since an unread pipe would eventually fill and
        # wedge it, so the task rides along for a caller that wants to report it.
        $process.StandardInput.Close()
        $process | Add-Member -PassThru -NotePropertyName StarshipErrors `
            -NotePropertyValue $process.StandardError.ReadToEndAsync()
    }

    function Invoke-Starship {
        $process = Start-StarshipProcess @args
        try {
            $output = $process.StandardOutput.ReadToEnd()
            # stderr isn't displayed with this style of invocation, so a bad
            # config would fail silently. Write it to the host ourselves; stdout
            # is already at EOF here, so awaiting the drain cannot deadlock.
            $errors = $process.StarshipErrors.GetAwaiter().GetResult()
            if ($errors.Trim()) { $host.ui.WriteErrorLine($errors) }
            $output
        } finally { $process.Dispose() }
    }

    function Stop-StarshipStream {
        if ($null -eq $script:StreamProcess) { return }
        # Killing the renderer closes its pipe, which ends the pump's read loop,
        # so EndInvoke both collects the timings and frees the engine to be
        # handed the next prompt's stream.
        try { $script:StreamProcess.Kill() } catch { }
        $script:StreamProcess.Dispose()
        $script:StreamProcess = $null
        # Unset only when the handshake threw before the pump was ever invoked.
        if ($script:PumpHandle) {
            $timings = $script:Engine.EndInvoke($script:PumpHandle)
            $script:PumpHandle = $null
            if ($timings.Count) { $script:StreamTimings = $timings[-1] }
        }
    }

    function Start-StarshipStream {
        try {
            # An empty --timings is the flag's own default, so the first prompt
            # of a session needs no special case.
            $script:StreamProcess = Start-StarshipProcess stream `
                "--timings=$($script:StreamTimings)" @args
            $ready = [Threading.Tasks.TaskCompletionSource[string]]::new()
            $script:Engine.Commands.Clear()
            $null = $script:Engine.AddCommand('Invoke-StarshipPump').
                AddArgument($script:StreamProcess.StandardOutput).
                AddArgument($script:Terminal).
                AddArgument($ready)
            $script:PumpHandle = $script:Engine.BeginInvoke()
            # The renderer has two seconds to paint, else the caller renders sync.
            if ($ready.Task.Wait(2000) -and $null -ne $ready.Task.Result) {
                return $ready.Task.Result
            }
        } catch { }
        Stop-StarshipStream
    }

    function Get-StarshipArguments($DollarQuestion, $LastExitCode) {
        # ProviderPath is physical only for FileSystem; the provider prefix is
        # what turns a logical Path into a physical one.
        $prefix = "$($PWD.Provider.ModuleName)\$($PWD.Provider.Name)::"
        # No history => success. A failed last command belongs to this line only
        # if $error[0] was raised by it — no error record leaves the comparison
        # against $null, which no real command line equals — and otherwise a
        # native exit code is the honest status.
        $last = Get-History -Count 1
        $status = if ($DollarQuestion -or -not $last) { 0 }
            elseif ($last.CommandLine -eq $global:error[0].InvocationInfo.Line) { 1 }
            else { $LastExitCode }
        @(
            "--path=$($PWD.ProviderPath)"
            "--logical-path=$(if ($PWD.Path.StartsWith($prefix)) { $PWD.Path.Substring($prefix.Length) } else { $PWD.Path })"
            "--terminal-width=$($Host.UI.RawUI.WindowSize.Width)"
            "--jobs=$(@(Get-Job -State Running).Count)"
            if ($last) {
                "--cmd-duration=$([math]::Round(($last.EndExecutionTime - $last.StartExecutionTime).TotalMilliseconds))"
            }
            "--status=$status"
            if ($script:LineEditor -and $script:LineEditor::InViCommandMode()) { '--keymap=vi' }
        )
    }

    function Enable-TransientPrompt {
        if (-not $script:LineEditor) { return }
        Set-PSReadLineKeyHandler -Key Enter -ScriptBlock {
            $enc = [Console]::OutputEncoding
            try {
                $parseErrors = $null
                $script:LineEditor::GetBufferState([ref]$null, [ref]$null, [ref]$parseErrors, [ref]$null)
                if ($parseErrors.Count -eq 0) {
                    Stop-StarshipStream
                    $script:TransientPrompt = $true
                    [Console]::OutputEncoding = [Text.Encoding]::UTF8
                    $script:LineEditor::InvokePrompt()
                }
            } finally {
                if ((Get-PSReadLineOption).PredictionViewStyle -eq 'ListView') {
                    # Clear prompt, input, and at most ten ListView prediction rows.
                    $n = [math]::Min($Host.UI.RawUI.WindowSize.Height - $Host.UI.RawUI.CursorPosition.Y - 1, 12)
                    $script:LineEditor::Insert("`n" * $n)
                    $script:LineEditor::Undo()
                }
                $script:LineEditor::AcceptLine()
                [Console]::OutputEncoding = $enc
            }
        }
    }

    function Disable-TransientPrompt {
        if ($script:LineEditor) { Set-PSReadLineKeyHandler -Key Enter -Function AcceptLine }
        $script:TransientPrompt = $false
    }

    function global:prompt {
        $origDollarQuestion = $global:?
        $origLastExitCode = $global:LASTEXITCODE
        Stop-StarshipStream
        try { if (${function:Invoke-Starship-PreCommand}) { Invoke-Starship-PreCommand } } catch { }
        $arguments = Get-StarshipArguments $origDollarQuestion $origLastExitCode
        $promptText = if ($script:TransientPrompt) {
            $script:TransientPrompt = $false
            if (${function:Invoke-Starship-TransientFunction}) {
                Invoke-Starship-TransientFunction
            } else {
                "$([char]27)[1;32m❯$([char]27)[0m "
            }
        } elseif ($script:Engine -and $null -ne ($streamed = Start-StarshipStream @arguments)) {
            $streamed
        } else {
            Invoke-Starship prompt @arguments
        }
        if ($script:LineEditor) {
            Set-PSReadLineOption -ExtraPromptLineCount ($promptText.Split("`n").Count - 1)
        }
        $promptText
        # $? cannot be assigned; forge success with a quiet expression, failure
        # with an ignored error record.
        $global:LASTEXITCODE = $origLastExitCode
        if ($global:? -ne $origDollarQuestion) {
            if ($origDollarQuestion) { $null = 1 + 1 } else { Write-Error '' -ErrorAction Ignore }
        }
    }

    $env:VIRTUAL_ENV_DISABLE_PROMPT = 1
    $env:STARSHIP_SHELL = ('powershell', 'pwsh')[$PSVersionTable.PSVersion.Major -gt 5]
    $env:STARSHIP_SESSION_KEY = [guid]::NewGuid().ToString('N').Substring(0, 16)

    if ($script:LineEditor) {
        Set-PSReadLineOption -ContinuationPrompt (Invoke-Starship prompt --continuation)
        try {
            # A mode change moves the prompt's own indicator, so redraw on top of
            # whatever handler was already installed. Script mode is what makes
            # PSReadLine call one at all, and is already set if one is there.
            $vi = (Get-PSReadLineOption).ViModeChangeHandler
            Set-PSReadLineOption -ViModeIndicator Script -ViModeChangeHandler {
                $script:LineEditor::InvokePrompt()
                if ($vi) { & $vi @args }
            }.GetNewClosure()
        } catch { }
    }

    $ExecutionContext.SessionState.Module.OnRemove = {
        Stop-StarshipStream
        if ($script:Engine) { $script:Engine.Runspace.Dispose(); $script:Engine.Dispose() }
        if ($script:Terminal) { $script:Terminal.Dispose() }
    }
    Export-ModuleMember Enable-TransientPrompt, Disable-TransientPrompt
}
