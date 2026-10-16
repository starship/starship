param([Parameter(Mandatory)][string]$Starship)
$ErrorActionPreference = 'Stop'

# Exercise the generated pump without loading interactive prompt hooks.
$source = (& $Starship init powershell --print-full-init) -join "`n"
if ($LASTEXITCODE -ne 0) { throw 'Starship initialization failed' }
$tokens = $null
$errors = $null
$ast = [Management.Automation.Language.Parser]::ParseInput($source, [ref]$tokens, [ref]$errors)
if ($errors.Count) { throw ($errors | Out-String) }
$assignment = $ast.Find({ param($node)
    $node -is [Management.Automation.Language.AssignmentStatementAst] -and
    $node.Left.Extent.Text -eq '$script:StreamPump'
}, $true)
$expression = $assignment.Right.Find({ param($node)
    $node -is [Management.Automation.Language.ScriptBlockExpressionAst]
}, $true)
$body = $expression.ScriptBlock.GetScriptBlock()
Add-Type @'
using System;
using System.IO;
using System.Threading;
using System.Threading.Tasks;
public sealed class SingleByteStream : MemoryStream {
    public SingleByteStream(byte[] bytes) : base(bytes) {}
    public override Task<int> ReadAsync(byte[] buffer, int offset, int count, CancellationToken token) {
        return base.ReadAsync(buffer, offset, Math.Min(count, 1), token);
    }
}
'@
function New-TestStream($reader) {
    [hashtable]::Synchronized(@{
        Process = [pscustomobject]@{ StandardOutput = $reader }
        Cancellation = [Threading.CancellationTokenSource]::new()
        Ready = [Threading.ManualResetEventSlim]::new()
        Prompt = $null
    })
}

foreach ($case in @(
    @{ Wire = ''; Prompt = $null; Refined = $false },
    @{ Wire = 'unfinished'; Prompt = $null; Refined = $false },
    @{ Wire = "main`0"; Prompt = $null; Refined = $false },
    @{ Wire = "`0`0"; Prompt = ''; Refined = $false },
    @{ Wire = "λ🙂`0right`0"; Prompt = 'λ🙂'; Refined = $false },
    @{ Wire = "λ🙂`0right`0complete 🙂`0right`0"; Prompt = 'complete 🙂'; Refined = $true },
    @{ Wire = "initial`0right`0`0`0"; Prompt = ''; Refined = $true }
)) {
    $bytes = [SingleByteStream]::new([Text.Encoding]::UTF8.GetBytes($case.Wire))
    $stream = New-TestStream ([IO.StreamReader]::new($bytes))
    # Records the events the pump raises.
    $events = [pscustomobject]@{ Raised = [Collections.Generic.List[string]]::new() } |
        Add-Member -PassThru ScriptMethod GenerateEvent { param($Name) $this.Raised.Add($Name) }
    & $body $stream $events
    if ($stream.Prompt -cne $case.Prompt) { throw "Read $($stream.Prompt), not $($case.Prompt)" }
    if (-not $stream.Ready.IsSet) { throw 'End of stream did not release the startup waiter' }
    $refined = @($events.Raised | Where-Object { $_ -eq 'Starship.Refined' })
    if (($refined.Count -eq 1) -ne $case.Refined) { throw "Refined $($refined.Count) times from $($case.Wire)" }
    $stream.Cancellation.Dispose()
    $stream.Ready.Dispose()
}

# Exercise startup's null-versus-empty decision and cleanup with the same pump.
foreach ($name in 'Start-StarshipStream', 'Stop-StarshipStream') {
    $definition = $ast.Find({ param($node)
        $node -is [Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -eq $name
    }, $true)
    . ([scriptblock]::Create($definition.Extent.Text))
}
function Start-Native {
    param($Executable, $Arguments)
    $script:TestProducer = [pscustomobject]@{
        StandardOutput = [IO.StreamReader]::new([SingleByteStream]::new([Text.Encoding]::UTF8.GetBytes($script:TestWire)))
        StandardError = [IO.StreamReader]::new([IO.MemoryStream]::new())
        Killed = $false
        Disposed = $false
    } | Add-Member -PassThru ScriptMethod Kill { $this.Killed = $true } |
        Add-Member -PassThru ScriptMethod Dispose {
            $this.StandardOutput.Dispose()
            $this.StandardError.Dispose()
            $this.Disposed = $true
        }
    $script:TestProducer
}
$script:StreamPump = $body
$script:Pump = [PowerShell]::Create()
$script:Unix = $false
try {
    foreach ($case in @(
        @{ Wire = ''; Prompt = $null },
        @{ Wire = "main`0"; Prompt = $null },
        @{ Wire = "`0`0"; Prompt = '' },
        @{ Wire = "main`0right`0"; Prompt = 'main' }
    )) {
        $script:TestWire = $case.Wire
        $prompt = Start-StarshipStream @('--status=0')
        if ($prompt -cne $case.Prompt) { throw 'Startup lost the difference between a missing and empty prompt' }
        if (($null -eq $script:Stream) -ne ($null -eq $case.Prompt)) { throw 'Startup cached an incomplete pair' }
        Stop-StarshipStream
        if (-not $script:TestProducer.Killed -or -not $script:TestProducer.Disposed) { throw 'Startup left its producer open' }
    }
} finally {
    Stop-StarshipStream
    $script:Pump.Dispose()
}

$info = [Diagnostics.ProcessStartInfo]::new('/bin/sleep', '30')
$info.UseShellExecute = $false
$info.RedirectStandardOutput = $true
$producer = [Diagnostics.Process]::Start($info)
$stream = New-TestStream $producer.StandardOutput
$pump = [PowerShell]::Create()
$null = $pump.AddScript($body).AddArgument($stream).AddArgument($null)
$pending = $pump.BeginInvoke()
try {
    Start-Sleep -Milliseconds 300
    $stream.Cancellation.Cancel()
    if (-not $pending.AsyncWaitHandle.WaitOne(5000)) { throw 'Cancellation remained blocked' }
    $null = $pump.EndInvoke($pending)
    if ($pump.HadErrors) { throw ($pump.Streams.Error | Out-String) }
    if (-not $stream.Ready.IsSet) { throw 'Cancellation did not release the startup waiter' }
} finally {
    if (-not $producer.HasExited) { $producer.Kill() }
    $producer.WaitForExit()
    $pump.Dispose()
    $producer.Dispose()
    $stream.Cancellation.Dispose()
    $stream.Ready.Dispose()
}
'PASS: split UTF-8, prompt startup and empty pairs, one refinement once complete, and cancellation'
