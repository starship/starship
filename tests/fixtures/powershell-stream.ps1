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
        Refined = $false
        Timings = $null
    })
}

# The misleading PID belongs to a separate process created by this test.
$sentinel = [Diagnostics.Process]::Start('/bin/sleep', '30')
try {
    $wire = [Text.Encoding]::UTF8.GetBytes("PROMPT`0λ🙂`0`0COMPLETE`0timings`0`0")
    $bytes = [SingleByteStream]::new($wire)
    $stream = New-TestStream ([IO.StreamReader]::new($bytes))
    $stream.Id = $sentinel.Id
    & $body $stream $null
    if ($stream.Prompt -cne 'λ🙂' -or $stream.Timings -cne 'timings') {
        throw 'Split UTF-8 or COMPLETE framing failed'
    }
    if ($bytes.CanRead) { throw 'COMPLETE did not close the owned reader' }
    $sentinel.Refresh()
    if ($sentinel.HasExited) { throw 'Unrelated sentinel terminated' }
    $stream.Cancellation.Dispose()
    $stream.Ready.Dispose()

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
} finally {
    if (-not $sentinel.HasExited) { $sentinel.Kill() }
    $sentinel.WaitForExit()
    $sentinel.Dispose()
}
'PASS: split UTF-8, COMPLETE, owned reader cancellation, and unrelated PID safety'
