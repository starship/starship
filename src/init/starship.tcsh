setenv STARSHIP_SHELL tcsh;
setenv STARSHIP_SESSION_KEY `::STARSHIP:: session`;

set USER_PRECMD = `alias precmd`;
set USER_POSTCMD = `alias postcmd`;

set STARSHIP_PRECMD = '\
    set STARSHIP_CMD_STATUS = $status; \
    set STARSHIP_DURATION = 0; \
    if ( $?STARSHIP_START_TIME ) then
        set STARSHIP_END_TIME = `::STARSHIP:: time`; \
        @ STARSHIP_DURATION = $STARSHIP_END_TIME - $STARSHIP_START_TIME; \
        unset STARSHIP_START_TIME; \
    endif'

set STARSHIP_POSTCMD = '\
    set STARSHIP_START_TIME = `::STARSHIP:: time`';

alias precmd "$STARSHIP_PRECMD; $USER_PRECMD";
alias postcmd "$STARSHIP_POSTCMD; $USER_POSTCMD";

set prompt = '`::STARSHIP:: prompt --status "$STARSHIP_CMD_STATUS" --cmd-duration "$STARSHIP_DURATION"`'; 
