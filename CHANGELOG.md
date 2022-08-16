# Changelog

## [1.10.1](https://github.com/starship/starship/compare/v1.10.0...v1.10.1) (2022-08-15)


### Bug Fixes

* Disable multithreading in `jwalk` (via `gitoxide`) as workaround for [#4251](https://github.com/starship/starship/issues/4251) ([#4258](https://github.com/starship/starship/issues/4258)) ([37b54f7](https://github.com/starship/starship/commit/37b54f7ac3ba53ea851b478501a96a7c4e188fc4))

## [1.10.0](https://github.com/starship/starship/compare/v1.9.1...v1.10.0) (2022-08-14)


### Features

* add bun module ([#4187](https://github.com/starship/starship/issues/4187)) ([85692d1](https://github.com/starship/starship/commit/85692d1bf6a8477b6879adaf8b51007389df4328))
* Add starship preset command ([#4112](https://github.com/starship/starship/issues/4112)) ([c8a5adb](https://github.com/starship/starship/commit/c8a5adb412e98b07017ffa0edea5554b0a23b840))
* Add support for blink, hidden, and strikethrough styles. ([#4138](https://github.com/starship/starship/issues/4138)) ([aaab920](https://github.com/starship/starship/commit/aaab920f88015eb0a44e6514bf19b1db2b14829f))
* Add the ability to have some file extensions *prevent* a module from triggering ([#4043](https://github.com/starship/starship/issues/4043)) ([dd73447](https://github.com/starship/starship/commit/dd73447329e637ee207b1103ecb6a4bdbdc89324))
* Enable transience for Cmd and PowerShell ([#4143](https://github.com/starship/starship/issues/4143)) ([6e9c013](https://github.com/starship/starship/commit/6e9c013e60e59660cb7ae6289af5ed129ca85996))
* **git:** replace `git2` with `git-repository` ([#3883](https://github.com/starship/starship/issues/3883)) ([ac55a01](https://github.com/starship/starship/commit/ac55a01d0ffe907ef7af48c9597c0bca4dbd8c69))
* **k8s:** Add folder detection to the k8s module. ([#4157](https://github.com/starship/starship/issues/4157)) ([5c5969c](https://github.com/starship/starship/commit/5c5969c50b2490309b7ae9f7e6f5f75ea04a512d))
* **package:** support cargo workspace versions ([#4161](https://github.com/starship/starship/issues/4161)) ([0a1235e](https://github.com/starship/starship/commit/0a1235e27944f152ca195c32e7eef8985d475989))
* **status:** Add pipestatus_segment_format option to status module ([#4103](https://github.com/starship/starship/issues/4103)) ([6143848](https://github.com/starship/starship/commit/61438484bdc76601a185298f14337cfb4d5b4e0b))


### Bug Fixes

* **aws:** support official `AWS_SHARED_CREDENTIALS_FILE` variable ([#4242](https://github.com/starship/starship/issues/4242)) ([1390036](https://github.com/starship/starship/commit/13900368826cf1aca160fd650f19cecc1a047372))
* **timings:** count time spent on custom on 'when' command failure ([#4121](https://github.com/starship/starship/issues/4121)) ([aae1ed0](https://github.com/starship/starship/commit/aae1ed04babf4c7d8baaad670c076947d7200675))


### Performance Improvements

* **pulumi:** allow disabling upwards discovery ([#4159](https://github.com/starship/starship/issues/4159)) ([af15de9](https://github.com/starship/starship/commit/af15de93c4494bb08d8c2cb3dbf54951f6bc9239))
* **rust:** avoid calling `rustup` in more conditions ([#4174](https://github.com/starship/starship/issues/4174)) ([d8ac940](https://github.com/starship/starship/commit/d8ac940098eb16417742723c627d0de864597410))


### Miscellaneous Chores

* **master:** release 1.10.0 ([b974610](https://github.com/starship/starship/commit/b9746100e2ee3746ea418e26ebc50494710a9d03))

## [1.9.1](https://github.com/starship/starship/compare/v1.9.0...v1.9.1) (2022-06-27)


### Bug Fixes

* regenerate changelog ([8a6be8c](https://github.com/starship/starship/commit/8a6be8c941de8e31330417bdb232204969a814ff))


### Miscellaneous Chores

* **master:** release 1.9.1 ([6cbc362](https://github.com/starship/starship/commit/6cbc3620c9ee7695fe1dabc067b6fd339b8d2526))

## [1.9.0](https://github.com/starship/starship/compare/v1.8.0...v1.9.0) (2022-06-26)


### Features

* add Raku module ([#4048](https://github.com/starship/starship/issues/4048)) ([1a4fac6](https://github.com/starship/starship/commit/1a4fac63f78c9408756c19eb26af5181a7cf537e))
* **winget:** Add support for winget package manager ([#4042](https://github.com/starship/starship/issues/4042)) ([ef52f9e](https://github.com/starship/starship/commit/ef52f9e77ec66f5189a18acfdce399882c37fdd8))


### Bug Fixes

* **character:** Standadise Vim config names ([#4081](https://github.com/starship/starship/issues/4081)) ([6761938](https://github.com/starship/starship/commit/67619386cdd7537f0ab9af77e701409e97a87917))
* **install:** Have fixed a spacing issue in output  ([#4082](https://github.com/starship/starship/issues/4082)) ([2ffe173](https://github.com/starship/starship/commit/2ffe1737f06db4ce89a21b2b5238f3ad76c94bca))

## [1.8.0](https://github.com/starship/starship/compare/v1.7.1...v1.8.0) (2022-06-16)


### Features

* Add support for Daml ([#4004](https://github.com/starship/starship/issues/4004)) ([3fe6cc0](https://github.com/starship/starship/commit/3fe6cc023cd52917ae60a4d06ee6f1f78baa19e7))
* **kubernetes:** add user alias ([#4008](https://github.com/starship/starship/issues/4008)) ([df5c2d8](https://github.com/starship/starship/commit/df5c2d8836622677460e34fa8082faa6b1a52835))
* **release:** add windows msi installers ([#4031](https://github.com/starship/starship/issues/4031)) ([89fd532](https://github.com/starship/starship/commit/89fd5320af248207e8b253790bd191d8daa88dbe))


### Bug Fixes

* escape text segments in meta variables ([#3563](https://github.com/starship/starship/issues/3563)) ([7d31bac](https://github.com/starship/starship/commit/7d31bac1cc3f39bd02f2e188e69283c566b816ed))
* **fish:** add proper vi mode detection for fish shell ([#3839](https://github.com/starship/starship/issues/3839)) ([1469763](https://github.com/starship/starship/commit/146976351ec804ab1594d5262a1e0dd2d2de4972))
* **install:** ignore tarfile ownership values when installing as root ([#4046](https://github.com/starship/starship/issues/4046)) ([1a91510](https://github.com/starship/starship/commit/1a91510beda1de2c3b149b7aacc0d76cf4652482))
* **nu:** don't use `cygpath` for starship binary path in init ([#4001](https://github.com/starship/starship/issues/4001)) ([9b52475](https://github.com/starship/starship/commit/9b52475e541f751e8c650587cd8c1615fe00b1d0))
* some typos ([e7c1976](https://github.com/starship/starship/commit/e7c19765282eb31daf85e5eba26e13828bc2f6c7))

### [1.7.1](https://github.com/starship/starship/compare/v1.7.0...v1.7.1) (2022-05-24)


### Bug Fixes

* trigger release ([2cdf902](https://github.com/starship/starship/commit/2cdf902b57cd16dba42d4026e2e740537a82b0ee))

## [1.7.0](https://github.com/starship/starship/compare/v1.6.3...v1.7.0) (2022-05-24)


### Features

* **go:** check for go.work file to show Go module in prompt ([#3968](https://github.com/starship/starship/issues/3968)) ([9ebfce1](https://github.com/starship/starship/commit/9ebfce1e366656bd1c199bb50cc7e1bd6cdb90ad))
* **hostname:** add `ssh_symbol` for ssh connections ([#3806](https://github.com/starship/starship/issues/3806)) ([2bf30dc](https://github.com/starship/starship/commit/2bf30dc89fbce6f4da37657b8af6077f15a543d0))
* **package:** Extract package version from PEP 621 compliant pyproject.toml ([#3950](https://github.com/starship/starship/issues/3950)) ([1b938fd](https://github.com/starship/starship/commit/1b938fd48420ceedf1e9886bd95ea738374680f7))
* **rust:** Display toolchain names ([#3414](https://github.com/starship/starship/issues/3414)) ([393d62c](https://github.com/starship/starship/commit/393d62c726573a0e6c351f422dbef4b17a5944bf))


### Bug Fixes

* **ci:** Version bump and fix Crowdin Pretranslate ([#3992](https://github.com/starship/starship/issues/3992)) ([a0a6c94](https://github.com/starship/starship/commit/a0a6c942fe3fc85d599aec883406224c9ecb589f))
* Do not panic in config if editor not found ([#3766](https://github.com/starship/starship/issues/3766)) ([2e80aec](https://github.com/starship/starship/commit/2e80aec5cb6f7376359e7a25a76a492a98717554))
* **module:** list option not working ([#3919](https://github.com/starship/starship/issues/3919)) ([6fe6735](https://github.com/starship/starship/commit/6fe6735927170b9f2aaa10cb84fa3a4d754e3bd6))
* **nu:** use the most recent starship init ([#3908](https://github.com/starship/starship/issues/3908)) ([382445d](https://github.com/starship/starship/commit/382445dc4d21d190959f5582fb9b9febe056299a))
* Use git2::Repository::open_ext() instead of discover() ([#3591](https://github.com/starship/starship/issues/3591)) ([81a696a](https://github.com/starship/starship/commit/81a696a914f6761d42b69f139018c3fa663ff197))

### [1.6.3](https://github.com/starship/starship/compare/v1.6.2...v1.6.3) (2022-04-26)


### Bug Fixes

* **git_branch:** correct variable name for remote branch ([#3897](https://github.com/starship/starship/issues/3897)) ([bd7957f](https://github.com/starship/starship/commit/bd7957f01c7fa2b14f068e4130f1aedea61f4a76))
* **schema:** move config-schema into docs folder ([#3878](https://github.com/starship/starship/issues/3878)) ([094f982](https://github.com/starship/starship/commit/094f982df184eecd85ea2832b3bf638629118c10))


### Performance Improvements

* **package:** only try to read files that exist ([#3904](https://github.com/starship/starship/issues/3904)) ([2a650bf](https://github.com/starship/starship/commit/2a650bfd140d561f955705cae124fb254ec549a1))


### Reverts

* **schema:** move config-schema back into .github folder ([#3886](https://github.com/starship/starship/issues/3886)) ([9b2ce42](https://github.com/starship/starship/commit/9b2ce4240c602df368f966996d870ef9197e65ac))

### [1.6.2](https://github.com/starship/starship/compare/v1.6.1...v1.6.2) (2022-04-15)


### Bug Fixes

* trigger another release ([81d2ce6](https://github.com/starship/starship/commit/81d2ce68ec05ea77151639267115d4daf73f5019))

### [1.6.1](https://github.com/starship/starship/compare/v1.6.0...v1.6.1) (2022-04-15)


### Bug Fixes

* fix release-please permissions ([23be606](https://github.com/starship/starship/commit/23be606516b5815fafea211aa59d2315ddb77df7))

## [1.6.0](https://github.com/starship/starship/compare/v1.5.4...v1.6.0) (2022-04-14)


### Features

* Add a module for C projects ([#3631](https://github.com/starship/starship/issues/3631)) ([0863146](https://github.com/starship/starship/commit/0863146f072ae8382be63db26dcf9ddeff967aea))
* allow printing config file schema ([#3737](https://github.com/starship/starship/issues/3737)) ([18ad26f](https://github.com/starship/starship/commit/18ad26f98dd1bfcc01e2b092a5b6165a7b093631))
* **aws:** add option to force AWS display ([#3720](https://github.com/starship/starship/issues/3720)) ([e04f126](https://github.com/starship/starship/commit/e04f126a107eba2e40009f21942c14894385d6b0))
* **cmd_duration:** make notify feature optional (compat with nix darwin) ([#3855](https://github.com/starship/starship/issues/3855)) ([efaab49](https://github.com/starship/starship/commit/efaab49e4753bee1ce90ad08311a1d4dc04052b8))
* **spack:** Add `Spack` module ([#3639](https://github.com/starship/starship/issues/3639)) ([3014284](https://github.com/starship/starship/commit/3014284e952f75deae87973cbe2763b7a0a0aab5))
* **username:** Detect Admin access in Windows ([#2791](https://github.com/starship/starship/issues/2791)) ([c89c130](https://github.com/starship/starship/commit/c89c13038a34a52291d253e6d4b15c0dd4aa5dfa))


### Bug Fixes

* **bash:** ensure `checkwinsize` is enabled for `$COLUMNS` ([#3832](https://github.com/starship/starship/issues/3832)) ([0334327](https://github.com/starship/starship/commit/03343272b778260016216266facd190936f9e7d3))
* **directory:** enable repo_root_style when truncation_length is zero. ([#3536](https://github.com/starship/starship/issues/3536)) ([441ebb3](https://github.com/starship/starship/commit/441ebb39b9cd451564959d259409d2395e7afb01))
* **docker_context:** ignore the "default" context ([#3803](https://github.com/starship/starship/issues/3803)) ([#3804](https://github.com/starship/starship/issues/3804)) ([230e85b](https://github.com/starship/starship/commit/230e85be37a0fc12999d1e6ff1209e7d5f99ecd1))
* **fish:** allow generating session keys in older versions of fish ([#3697](https://github.com/starship/starship/issues/3697)) ([0fb4219](https://github.com/starship/starship/commit/0fb421969058ec07a09f7c927dddc1258de75631))
* **init:** Change Elvish init to `catch` for 0.18 ([#3769](https://github.com/starship/starship/issues/3769)) ([538329d](https://github.com/starship/starship/commit/538329d9b406cd6358d0fe31d58e0c9f578ceffa))
* **nu:** Use `=` instead of space to pass command line parameters ([#3833](https://github.com/starship/starship/issues/3833)) ([2608db3](https://github.com/starship/starship/commit/2608db3a38b0dca13d91e94950fb4246b0ed1d82))
* **nu:** use shell-provided terminal width ([#3800](https://github.com/starship/starship/issues/3800)) ([859b780](https://github.com/starship/starship/commit/859b780b46780fdcac8141a9d165066880c36261))
