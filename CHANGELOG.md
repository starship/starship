# Changelog

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
