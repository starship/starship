# Starship tasks

## lint

> Lint the project with `clippy`

```sh
cargo clippy
```

## format

> Format all source files using `rustfmt`

**OPTIONS**

- check
  - flags: -c --check
  - desc: Show which files are not formatted correctly

```sh
if [ "$check" = "true" ]; then
  cargo fmt --all -- --check
else
  cargo fmt
fi
```

## script

> Commands related to shell scripts

### script lint (script)

> Lint shell script using `shellcheck`.

```sh
# lint zsh scripts as bash
case "$script" in
  *.sh) shell="sh" ;;
  *.bash) shell="bash" ;;
  *.ksh) shell="ksh" ;;
  *.zsh) shell="bash" ;;

  *)
    printf "error: Unsupported file: %s\n" "$script" >&2
    exit 1
    ;;
esac

cmd="shellcheck --shell=$shell $script"
printf "\n%s: %s\n" "$script" "$cmd"
$cmd
```

### script format (script)

> Format shell script using `shfmt`.

```sh
# Flags:
#   -i 2 - indent using 2 spaces
#   -ci  - indent `case` blocks
#   -w   - overwrite
flags="-i 2 -ci -w"

case "$script" in
  *.sh) shell="posix" ;;
  *.bash) shell="bash" ;;
  *.ksh) shell="mksh" ;;

  *)
    printf "error: Unsupported file: %s\n" "$script" >&2
    exit 1
    ;;
esac

cmd="shfmt -ln=$shell $flags $script"
printf "\n%s: %s\n" "$script" "$cmd"
$cmd && printf "Formatted: %s\n" "$script"
```
