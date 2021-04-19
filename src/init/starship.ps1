#!/usr/bin/env pwsh

function global:prompt {
    $origDollarQuestion = $global:?
    $origLastExitCode = $global:LASTEXITCODE

    $out = $null
    # @ makes sure the result is an array even if single or no values are returned
    $jobs = @(Get-Job | Where-Object { $_.State -eq 'Running' }).Count

    $env:PWD = $PWD
    $current_directory = (Convert-Path -LiteralPath $PWD)
    
    # Whe start from the premise that the command executed correctly, which covers also the fresh console.
    $lastExitCodeForPrompt = 0

    # Save old output encoding and set it to UTF-8
    $origOutputEncoding = [Console]::OutputEncoding
    [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
    if ($lastCmd = Get-History -Count 1) {
        # In case we have a False on the Dollar hook, we know there's an error.
        if( -not $origDollarQuestion){
          # We retrieve the InvocationInfo from the most recent error.
          $lastCmdletError = Get-Error |  Where-Object {$_ -ne $null} | Select-Object -expand InvocationInfo
          # We check if the las command executed matches the line that caused the last error , in which case we know
          # it was an internal Powershell command, otherwise, there MUST be an error code.
          $lastExitCodeForPrompt = if($lastCmd.CommandLine -eq $lastCmdletError.Line){1} else {$origLastExitCode}
        }

        $duration = [math]::Round(($lastCmd.EndExecutionTime - $lastCmd.StartExecutionTime).TotalMilliseconds)
        # & ensures the path is interpreted as something to execute
        $out = @(&::STARSHIP:: prompt "--path=$current_directory" --status=$lastExitCodeForPrompt --jobs=$jobs --cmd-duration=$duration)
    } else {
        $out = @(&::STARSHIP:: prompt "--path=$current_directory" --status=$lastExitCodeForPrompt --jobs=$jobs)
    }
    # Restore old output encoding
    [Console]::OutputEncoding = $origOutputEncoding

    # Convert stdout (array of lines) to expected return type string
    # `n is an escaped newline
    $out -join "`n"

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

$ENV:STARSHIP_SHELL = "powershell"

# Set up the session key that will be used to store logs
$ENV:STARSHIP_SESSION_KEY = (& ::STARSHIP:: session)
