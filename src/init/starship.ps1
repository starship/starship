#!/usr/bin/env pwsh

# Starship assumes UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
function global:prompt {
    $origDollarQuestion = $global:?
    $origLastExitCode = $global:LASTEXITCODE
    $origCaretValue = ${global:^}
    $origLastErrorCmdletName = Get-Error |  Where-Object {$_ -ne $null} | Select-Object -expand InvocationInfo | Where-Object {$_.InvocationName -ne $null} | Select-Object -expand InvocationName

    $out = $null
    # @ makes sure the result is an array even if single or no values are returned
    $jobs = @(Get-Job | Where-Object { $_.State -eq 'Running' }).Count

    $env:PWD = $PWD
    $current_directory = (Convert-Path -LiteralPath $PWD)
    
    # If an external command has not been executed, then the $LASTEXITCODE will be $null.
    # it isn't populated or modified until you execute a Powershell task that returns an exit
    # code e.g. On a fresh Powershell console:
    # > $LASTEXITCODE -eq $null
    # True
    # In the same console after running a Powershell cmdlet:
    # > gci ~/non_existing_folder
    # ...
    # > $LASTEXITCODE -eq $null
    # True
    # It only gets populated when running an external executable that returns an exit code, e. g.
    # > bash -c 'exit 5'
    # > $LASTEXITCODE -eq $null
    # False
    # > $LASTEXITCODE
    # 5
    # Whe start from the premise that the command executed correctly, which covers also the fresh console.
    $lastExitCodeForPrompt = 0

    # In case we have a False on the Dollar hook, we know there's an error.
    if( -not $origDollarQuestion){
    # We check if the caret (^) original value is null aka. this is the first command executed.
      if ($origCaretValue -eq $null){
    # If there's a $LASTEXITCODE we use it, otherwise, we can safely return the standard error.
        $lastExitCodeForPrompt = @($origLastExitCode,{1})[$origLastExitCode -eq $null]
      }else{
    # Finally if the last error origin is the same as the last command run, we know it was an internal
    # Powershell command and we can return 1, otherwise, we use the $LASTEXITCODE
        $lastExitCodeForPrompt = @({1},$origLastExitCode)[$origCaretValue -eq $origLastErrorCmdletName]
      }
    }



    if ($lastCmd = Get-History -Count 1) {
        $duration = [math]::Round(($lastCmd.EndExecutionTime - $lastCmd.StartExecutionTime).TotalMilliseconds)
        # & ensures the path is interpreted as something to execute
        $out = @(&::STARSHIP:: prompt "--path=$current_directory" --status=$lastExitCodeForPrompt --jobs=$jobs --cmd-duration=$duration)
    } else {
        $out = @(&::STARSHIP:: prompt "--path=$current_directory" --status=$lastExitCodeForPrompt --jobs=$jobs)
    }

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
