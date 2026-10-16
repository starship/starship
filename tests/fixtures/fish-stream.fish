source "$STARSHIP_INIT"

set -g test_stream_calls 0
set -g test_main_calls 0
set -g test_right_calls 0
set -g test_directory "$PWD/stream"
mkdir "$test_directory"

function expect --argument-names actual expected
    if test "$actual" != "$expected"
        printf 'Expected <%s>, got <%s>\n' "$expected" "$actual" >&2
        exit 1
    end
end

function __starship_test_command
    if contains -- --stream $argv
        set -g test_stream_calls (math $test_stream_calls + 1)
        switch "$test_failure"
            case no-directory
                return 1
            case missing-directory
                printf '%s\0' "$test_directory/missing"
            case '*'
                printf '%s\0' "$test_directory"
        end
    else if contains -- --right $argv
        set -g test_right_calls (math $test_right_calls + 1)
        printf RIGHT_FALLBACK
    else
        set -g test_main_calls (math $test_main_calls + 1)
        printf MAIN_FALLBACK
    end
end

set -l arguments --status=0 --terminal-width=80
for failure in no-directory missing-directory incomplete-pair
    set -g test_failure $failure
    printf 'incomplete\0' >"$test_directory/prompt"
    set -l streams $test_stream_calls
    expect (__starship_prompt 1 $arguments) MAIN_FALLBACK
    expect (__starship_prompt 2 $arguments) RIGHT_FALLBACK
    expect "$test_stream_calls" (math $streams + 2)
    set -q __starship_stream_arguments; and exit 3
end
expect "$test_main_calls" 3
expect "$test_right_calls" 3

# Both fields can be empty without indicating failure or forcing a restart.
set -g test_failure ''
printf '\0\0' >"$test_directory/prompt"
set -l drawn_main (__starship_prompt 1 $arguments | string collect)
set -l drawn_right (__starship_prompt 2 $arguments | string collect)
expect "$drawn_main" ''
expect "$drawn_right" ''
expect "$test_stream_calls" 7
set -q __starship_stream_arguments; or exit 4
expect "$test_main_calls" 3
expect "$test_right_calls" 3

__starship_stream_stop
printf 'MAIN_CACHED\0RIGHT_CACHED\0' >"$test_directory/prompt"
expect (__starship_prompt 1 $arguments) MAIN_CACHED
expect (__starship_prompt 2 $arguments) RIGHT_CACHED
expect (__starship_prompt 1 $arguments) MAIN_CACHED
expect "$test_stream_calls" 8

# Quoted Fish lists compare as scalar values, including their length.
set arguments --status=0 --terminal-width=120
expect (__starship_prompt 1 $arguments) MAIN_CACHED
expect (__starship_prompt 2 $arguments) RIGHT_CACHED
expect "$test_stream_calls" 9
set -a arguments --jobs=2
expect (__starship_prompt 2 $arguments) RIGHT_CACHED
expect "$test_stream_calls" 10

# Each side draws the newest pair, read as it is drawn into variables of
# its own, leaving the user's alone.
set -g main USER_MAIN
set -g right USER_RIGHT
printf 'MAIN_NEWER\0RIGHT_NEWER\0' >"$test_directory/prompt"
expect (__starship_prompt 1 $arguments) MAIN_NEWER
expect (__starship_prompt 2 $arguments) RIGHT_NEWER
expect "$main $right" "USER_MAIN USER_RIGHT"

# A partial update must leave the last complete pair intact.
printf 'partial\0' >"$test_directory/prompt"
__starship_stream_read; and exit 2
expect (__starship_prompt 1 $arguments) MAIN_NEWER
expect (__starship_prompt 2 $arguments) RIGHT_NEWER
expect "$test_stream_calls" 10
__starship_stream_stop
set -q __starship_stream_arguments; and exit 5

# An empty argument list also needs an initial stream, then shares its cache.
printf '\0\0' >"$test_directory/prompt"
__starship_prompt 1
__starship_prompt 2
expect "$test_stream_calls" 11
set -q __starship_stream_arguments; or exit 6
__starship_stream_stop
printf 'PASS: stream fallback and prompt pair caching\n'
