setenv STARSHIP_SHELL tcsh;
setenv STARSHIP_SESSION_KEY `(::STARSHIP:: session)`;
alias precmd 'set STARSHIP_CMD_STATUS = $status;set STARSHIP_PATH = ::STARSHIP::;set NUM_JOBS=`(jobs | wc -l)`;set STARSHIP_END_TIME = `$STARSHIP_PATH time`;set STARSHIP_DURATION = 0;if ( $?STARSHIP_START_TIME ) @ STARSHIP_DURATION = $STARSHIP_END_TIME - $STARSHIP_START_TIME;set prompt = "`($STARSHIP_PATH prompt --status $STARSHIP_CMD_STATUS --jobs $NUM_JOBS --cmd-duration $STARSHIP_DURATION)`";unset STARSHIP_START_TIME;';
alias postcmd 'set STARSHIP_START_TIME = `(::STARSHIP:: time)`';
set STARSHIP_START_TIME = `(::STARSHIP:: time)`;
