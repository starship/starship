export STARSHIP_SHELL="ksh"
STARSHIP_SESSION_KEY="$(::STARSHIP:: time)$RANDOM"
export STARSHIP_SESSION_KEY="${STARSHIP_SESSION_KEY:0:16}"
PS1='$(::STARSHIP:: prompt --status $? --terminal-width "${COLUMNS:-80}")'
PS2="$(::STARSHIP:: prompt --continuation)"
