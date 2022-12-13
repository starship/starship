# Changelog

## [1.10.0](https://github.com/starship/starship/compare/v1.12.0...v1.10.0) (2022-12-13)


### Features

* Add a module for C projects ([#3631](https://github.com/starship/starship/issues/3631)) ([0863146](https://github.com/starship/starship/commit/0863146f072ae8382be63db26dcf9ddeff967aea))
* add bun module ([#4187](https://github.com/starship/starship/issues/4187)) ([85692d1](https://github.com/starship/starship/commit/85692d1bf6a8477b6879adaf8b51007389df4328))
* add Haxe support ([#4395](https://github.com/starship/starship/issues/4395)) ([2766c78](https://github.com/starship/starship/commit/2766c78749e638282d1dee56f7afcc195c16c064))
* Add operating system module ([#4109](https://github.com/starship/starship/issues/4109)) ([3109943](https://github.com/starship/starship/commit/3109943822a15b22faaa6cdfda17ca9554bcd800))
* add Raku module ([#4048](https://github.com/starship/starship/issues/4048)) ([1a4fac6](https://github.com/starship/starship/commit/1a4fac63f78c9408756c19eb26af5181a7cf537e))
* Add starship preset command ([#4112](https://github.com/starship/starship/issues/4112)) ([c8a5adb](https://github.com/starship/starship/commit/c8a5adb412e98b07017ffa0edea5554b0a23b840))
* Add support for blink, hidden, and strikethrough styles. ([#4138](https://github.com/starship/starship/issues/4138)) ([aaab920](https://github.com/starship/starship/commit/aaab920f88015eb0a44e6514bf19b1db2b14829f))
* Add support for Daml ([#4004](https://github.com/starship/starship/issues/4004)) ([3fe6cc0](https://github.com/starship/starship/commit/3fe6cc023cd52917ae60a4d06ee6f1f78baa19e7))
* Add the ability to have some file extensions *prevent* a module from triggering ([#4043](https://github.com/starship/starship/issues/4043)) ([dd73447](https://github.com/starship/starship/commit/dd73447329e637ee207b1103ecb6a4bdbdc89324))
* add user-defined color palette ([#4209](https://github.com/starship/starship/issues/4209)) ([d93074d](https://github.com/starship/starship/commit/d93074d0569db4bafb1788aa3f39136b734b5370))
* allow printing config file schema ([#3737](https://github.com/starship/starship/issues/3737)) ([18ad26f](https://github.com/starship/starship/commit/18ad26f98dd1bfcc01e2b092a5b6165a7b093631))
* **aws:** add a fallback for `expiration` key ([#4455](https://github.com/starship/starship/issues/4455)) ([5a2c85d](https://github.com/starship/starship/commit/5a2c85d078c1a8c83cc055dd0e56033abb15c2bf))
* **aws:** add option to force AWS display ([#3720](https://github.com/starship/starship/issues/3720)) ([e04f126](https://github.com/starship/starship/commit/e04f126a107eba2e40009f21942c14894385d6b0))
* **aws:** Add profile aliases ([#3699](https://github.com/starship/starship/issues/3699)) ([ac8c2fe](https://github.com/starship/starship/commit/ac8c2fe02474bee6fa41abf826501ec663cb0bb0))
* **azure:** add username to azure module config ([#4323](https://github.com/starship/starship/issues/4323)) ([6e15c00](https://github.com/starship/starship/commit/6e15c00238a06e92cf411a669590002eb22324e7))
* **buf:** Add Buf module ([#3661](https://github.com/starship/starship/issues/3661)) ([16f62d7](https://github.com/starship/starship/commit/16f62d790431ba2dd1fd02199a6924c00f6516d0))
* **bug-report:** ask for confirmation before opening issue ([#4543](https://github.com/starship/starship/issues/4543)) ([8bb9038](https://github.com/starship/starship/commit/8bb9038431cd369e953ca156ed09aabd7c2ba326))
* **cmd_duration:** make notify feature optional (compat with nix darwin) ([#3855](https://github.com/starship/starship/issues/3855)) ([efaab49](https://github.com/starship/starship/commit/efaab49e4753bee1ce90ad08311a1d4dc04052b8))
* **directory:** add before_repo_root_style ([#4595](https://github.com/starship/starship/issues/4595)) ([ea6249b](https://github.com/starship/starship/commit/ea6249b5243acf0cce2352a1b580479546b92340))
* Enable transience for Cmd and PowerShell ([#4143](https://github.com/starship/starship/issues/4143)) ([6e9c013](https://github.com/starship/starship/commit/6e9c013e60e59660cb7ae6289af5ed129ca85996))
* **fish:** Enable left and right transience ([#4204](https://github.com/starship/starship/issues/4204)) ([06281c2](https://github.com/starship/starship/commit/06281c268d74a85d5b28e953bea251a2115f5568))
* **git_branch:** add 'ignore_branches' option ([#3753](https://github.com/starship/starship/issues/3753)) ([bae16b5](https://github.com/starship/starship/commit/bae16b525de1f05a7ad125b5f4a8cb8baa7d5fae))
* **git_commit:** support showing lightweight tags ([#4632](https://github.com/starship/starship/issues/4632)) ([ac37792](https://github.com/starship/starship/commit/ac37792c19d7c545d4c51cf712f13e5e81559511))
* **git:** replace `git2` with `git-repository` ([#3883](https://github.com/starship/starship/issues/3883)) ([ac55a01](https://github.com/starship/starship/commit/ac55a01d0ffe907ef7af48c9597c0bca4dbd8c69))
* **go:** check for go.work file to show Go module in prompt ([#3968](https://github.com/starship/starship/issues/3968)) ([9ebfce1](https://github.com/starship/starship/commit/9ebfce1e366656bd1c199bb50cc7e1bd6cdb90ad))
* **guix_shell:** Initial implementation ([#4397](https://github.com/starship/starship/issues/4397)) ([d4bcc51](https://github.com/starship/starship/commit/d4bcc519e61524e1fe30f82412a09af113d75287))
* **haskell:** Add Haskell module ([#3587](https://github.com/starship/starship/issues/3587)) ([72fec55](https://github.com/starship/starship/commit/72fec559c524247f4f92749dfd6702fcb3d97484))
* **hostname:** add `ssh_symbol` for ssh connections ([#3806](https://github.com/starship/starship/issues/3806)) ([2bf30dc](https://github.com/starship/starship/commit/2bf30dc89fbce6f4da37657b8af6077f15a543d0))
* **init:** Use which-rs to resolve starship path ([cc2c8c4](https://github.com/starship/starship/commit/cc2c8c4a5450f2811612129abfbdc1aba12def91))
* **k8s:** Add folder detection to the k8s module. ([#4157](https://github.com/starship/starship/issues/4157)) ([5c5969c](https://github.com/starship/starship/commit/5c5969c50b2490309b7ae9f7e6f5f75ea04a512d))
* **kubernetes:** add user alias ([#4008](https://github.com/starship/starship/issues/4008)) ([df5c2d8](https://github.com/starship/starship/commit/df5c2d8836622677460e34fa8082faa6b1a52835))
* **localip:** use reserved remote address ([#4648](https://github.com/starship/starship/issues/4648)) ([ddd54e9](https://github.com/starship/starship/commit/ddd54e9b20427b716e13d83884b4b0db03953210)), closes [#4614](https://github.com/starship/starship/issues/4614)
* **module:** Add a meson devenv indicator ([#4389](https://github.com/starship/starship/issues/4389)) ([355800f](https://github.com/starship/starship/commit/355800f8147b1755a5289dc679e2147abd662daf))
* **nodejs:** check for `.mts` and `.cts` files ([#3734](https://github.com/starship/starship/issues/3734)) ([a10e24b](https://github.com/starship/starship/commit/a10e24b2052047d431b6a44b0a202f605c39bc96))
* **nu:** enable right prompt ([#4490](https://github.com/starship/starship/issues/4490)) ([a7abc0f](https://github.com/starship/starship/commit/a7abc0f4508b5357e44bc1d0a8b0ed363201824c)), closes [#3982](https://github.com/starship/starship/issues/3982)
* Open Policy Agent module ([#1740](https://github.com/starship/starship/issues/1740)) ([#4441](https://github.com/starship/starship/issues/4441)) ([865e68d](https://github.com/starship/starship/commit/865e68da3ad752a2bc85b923258f2dbd5287ada8))
* **package:** added showing gradle version based on the gradle.properties file ([#4432](https://github.com/starship/starship/issues/4432)) ([14ee81b](https://github.com/starship/starship/commit/14ee81b9c31047993217f060b57fb327a58c0d38))
* **package:** Extract package version from PEP 621 compliant pyproject.toml ([#3950](https://github.com/starship/starship/issues/3950)) ([1b938fd](https://github.com/starship/starship/commit/1b938fd48420ceedf1e9886bd95ea738374680f7))
* **package:** support cargo workspace versions ([#4161](https://github.com/starship/starship/issues/4161)) ([0a1235e](https://github.com/starship/starship/commit/0a1235e27944f152ca195c32e7eef8985d475989))
* **preset:** Add No Empty Icons preset ([#4518](https://github.com/starship/starship/issues/4518)) ([1a3d51f](https://github.com/starship/starship/commit/1a3d51fe76c5a62d53533f5d14ceb4425d5a33a5))
* **preset:** Add no-nerd-font preset ([#4517](https://github.com/starship/starship/issues/4517)) ([4d86a4c](https://github.com/starship/starship/commit/4d86a4c7ae70dff552cdea85d7ea7872e2321c2f))
* **release:** add chocolatey publishing ([#4637](https://github.com/starship/starship/issues/4637)) ([df37e8d](https://github.com/starship/starship/commit/df37e8d40c7b3556f8428ce29c53f2882af2ce25))
* **release:** add windows msi installers ([#4031](https://github.com/starship/starship/issues/4031)) ([89fd532](https://github.com/starship/starship/commit/89fd5320af248207e8b253790bd191d8daa88dbe))
* **rust:** Display toolchain names ([#3414](https://github.com/starship/starship/issues/3414)) ([393d62c](https://github.com/starship/starship/commit/393d62c726573a0e6c351f422dbef4b17a5944bf))
* **schema:** deny unknown keys ([#4270](https://github.com/starship/starship/issues/4270)) ([b5d3d8f](https://github.com/starship/starship/commit/b5d3d8fcf331cdff6d0e687dcdbac77351731475))
* **spack:** Add `Spack` module ([#3639](https://github.com/starship/starship/issues/3639)) ([3014284](https://github.com/starship/starship/commit/3014284e952f75deae87973cbe2763b7a0a0aab5))
* **status:** Add pipestatus_segment_format option to status module ([#4103](https://github.com/starship/starship/issues/4103)) ([6143848](https://github.com/starship/starship/commit/61438484bdc76601a185298f14337cfb4d5b4e0b))
* **status:** Support formatting of pipestatus separator ([#4264](https://github.com/starship/starship/issues/4264)) ([6e35dfa](https://github.com/starship/starship/commit/6e35dfa85aeebb3f714389a9286623dc0f60d799))
* **username:** Detect Admin access in Windows ([#2791](https://github.com/starship/starship/issues/2791)) ([c89c130](https://github.com/starship/starship/commit/c89c13038a34a52291d253e6d4b15c0dd4aa5dfa))
* **winget:** Add support for winget package manager ([#4042](https://github.com/starship/starship/issues/4042)) ([ef52f9e](https://github.com/starship/starship/commit/ef52f9e77ec66f5189a18acfdce399882c37fdd8))


### Bug Fixes

* **aws:** accept sso credentials ([#3718](https://github.com/starship/starship/issues/3718)) ([d730820](https://github.com/starship/starship/commit/d7308203a92bc067a3cb5177a5c6c32981c40959))
* **aws:** enable when using .aws/credentials ([#4604](https://github.com/starship/starship/issues/4604)) ([c8ac877](https://github.com/starship/starship/commit/c8ac8777a593358868813254c662da5fcb9fe6c8))
* **aws:** Make AWS_REGION orverrides AWS_DEFAULT_REGION ([#3619](https://github.com/starship/starship/issues/3619)) ([#3733](https://github.com/starship/starship/issues/3733)) ([59622bc](https://github.com/starship/starship/commit/59622bc41b5cf71e76d45b97681db298e8230656))
* **aws:** support official `AWS_SHARED_CREDENTIALS_FILE` variable ([#4242](https://github.com/starship/starship/issues/4242)) ([1390036](https://github.com/starship/starship/commit/13900368826cf1aca160fd650f19cecc1a047372))
* **bash:** ensure `checkwinsize` is enabled for `$COLUMNS` ([#3832](https://github.com/starship/starship/issues/3832)) ([0334327](https://github.com/starship/starship/commit/03343272b778260016216266facd190936f9e7d3))
* **buf:** broken icon on windows 10 ([#4689](https://github.com/starship/starship/issues/4689)) ([7341607](https://github.com/starship/starship/commit/7341607c294a633477005d777bd03b18522aabf4))
* **buf:** fix spacing & harmonize docs with actual configuration ([#4450](https://github.com/starship/starship/issues/4450)) ([3d45236](https://github.com/starship/starship/commit/3d452367bdde22a2554cc74bee4d1adfee7e8e04))
* **character:** Standadise Vim config names ([#4081](https://github.com/starship/starship/issues/4081)) ([6761938](https://github.com/starship/starship/commit/67619386cdd7537f0ab9af77e701409e97a87917))
* **ci:** cache after selecting the toolchain ([#4619](https://github.com/starship/starship/issues/4619)) ([e4dbff0](https://github.com/starship/starship/commit/e4dbff0fc7e88f792b90703f03f83e31d401b90e))
* **ci:** Version bump and fix Crowdin Pretranslate ([#3992](https://github.com/starship/starship/issues/3992)) ([a0a6c94](https://github.com/starship/starship/commit/a0a6c942fe3fc85d599aec883406224c9ecb589f))
* **config:** unrecognized config properties don't cause config error ([#4547](https://github.com/starship/starship/issues/4547)) ([1b03ef2](https://github.com/starship/starship/commit/1b03ef21f34fc4acf890b01cfca3d07158ef5c46))
* **container:** avoid detecting WSL as a systemd-container ([#4593](https://github.com/starship/starship/issues/4593)) ([b47a4fe](https://github.com/starship/starship/commit/b47a4fe51470a36116b5c941c6e07ac5730585ea))
* **directory:** don't strip duplicate directory names twice ([#4295](https://github.com/starship/starship/issues/4295)) ([801fbab](https://github.com/starship/starship/commit/801fbab720e1bb94c32bb1aa10966a0637a10e63))
* **directory:** enable repo_root_style when truncation_length is zero. ([#3536](https://github.com/starship/starship/issues/3536)) ([441ebb3](https://github.com/starship/starship/commit/441ebb39b9cd451564959d259409d2395e7afb01))
* Disable multithreading in `jwalk` (via `gitoxide`) as workaround for [#4251](https://github.com/starship/starship/issues/4251) ([#4258](https://github.com/starship/starship/issues/4258)) ([37b54f7](https://github.com/starship/starship/commit/37b54f7ac3ba53ea851b478501a96a7c4e188fc4))
* Do not panic in config if editor not found ([#3766](https://github.com/starship/starship/issues/3766)) ([2e80aec](https://github.com/starship/starship/commit/2e80aec5cb6f7376359e7a25a76a492a98717554))
* **docker_context:** ignore the "default" context ([#3803](https://github.com/starship/starship/issues/3803)) ([#3804](https://github.com/starship/starship/issues/3804)) ([230e85b](https://github.com/starship/starship/commit/230e85be37a0fc12999d1e6ff1209e7d5f99ecd1))
* **docs:** fix and cleanup VuePress config ([#3738](https://github.com/starship/starship/issues/3738)) ([7cdc230](https://github.com/starship/starship/commit/7cdc230100dc7208d9bb4957b0c01a65b7db60c0))
* don't attempt to display cmd_duration notification if in TTY ([#4535](https://github.com/starship/starship/issues/4535)) ([0427863](https://github.com/starship/starship/commit/04278631687da388005f2c26f3da2115b9075bf5))
* escape text segments in meta variables ([#3563](https://github.com/starship/starship/issues/3563)) ([7d31bac](https://github.com/starship/starship/commit/7d31bac1cc3f39bd02f2e188e69283c566b816ed))
* **fish:** add proper vi mode detection for fish shell ([#3839](https://github.com/starship/starship/issues/3839)) ([1469763](https://github.com/starship/starship/commit/146976351ec804ab1594d5262a1e0dd2d2de4972))
* **fish:** allow generating session keys in older versions of fish ([#3697](https://github.com/starship/starship/issues/3697)) ([0fb4219](https://github.com/starship/starship/commit/0fb421969058ec07a09f7c927dddc1258de75631))
* fix release-please permissions ([23be606](https://github.com/starship/starship/commit/23be606516b5815fafea211aa59d2315ddb77df7))
* **git_branch:** correct variable name for remote branch ([#3897](https://github.com/starship/starship/issues/3897)) ([bd7957f](https://github.com/starship/starship/commit/bd7957f01c7fa2b14f068e4130f1aedea61f4a76))
* **git:** check `tag_disabled` option ([#4527](https://github.com/starship/starship/issues/4527)) ([fd165b9](https://github.com/starship/starship/commit/fd165b96cc9587f81ab68b580d371b71f4e0ff35))
* **git:** upgrade `gitoxide` to v0.21 ([#4277](https://github.com/starship/starship/issues/4277)) ([f52ae55](https://github.com/starship/starship/commit/f52ae552d3ef2c0c0c6de6429cee7b8271f14671))
* **init:** Change Elvish init to `catch` for 0.18 ([#3769](https://github.com/starship/starship/issues/3769)) ([538329d](https://github.com/starship/starship/commit/538329d9b406cd6358d0fe31d58e0c9f578ceffa))
* **init:** Change Nushell init for nu 0.60 ([#3773](https://github.com/starship/starship/issues/3773)) ([c9b75fe](https://github.com/starship/starship/commit/c9b75fe115075c23eb456df5b1af8f4491834aaf))
* **install:** Add -o flag to unzip to match tar ([#3727](https://github.com/starship/starship/issues/3727)) ([ef96727](https://github.com/starship/starship/commit/ef967271e6009d3515fdd4c3dd64f575676411a9))
* **install:** Have fixed a spacing issue in output  ([#4082](https://github.com/starship/starship/issues/4082)) ([2ffe173](https://github.com/starship/starship/commit/2ffe1737f06db4ce89a21b2b5238f3ad76c94bca))
* **install:** ignore tarfile ownership values when installing as root ([#4046](https://github.com/starship/starship/issues/4046)) ([1a91510](https://github.com/starship/starship/commit/1a91510beda1de2c3b149b7aacc0d76cf4652482))
* **java:** Improved regex for Java version (starship[#4610](https://github.com/starship/starship/issues/4610)) ([#4616](https://github.com/starship/starship/issues/4616)) ([a9eb65e](https://github.com/starship/starship/commit/a9eb65ef35de948880cbf340ffbfe6af126e5e44))
* **module:** list option not working ([#3919](https://github.com/starship/starship/issues/3919)) ([6fe6735](https://github.com/starship/starship/commit/6fe6735927170b9f2aaa10cb84fa3a4d754e3bd6))
* **nu:** don't use `cygpath` for starship binary path in init ([#4001](https://github.com/starship/starship/issues/4001)) ([9b52475](https://github.com/starship/starship/commit/9b52475e541f751e8c650587cd8c1615fe00b1d0))
* **nu:** remove -c parameter from `term size` ([#4477](https://github.com/starship/starship/issues/4477)) ([4999530](https://github.com/starship/starship/commit/49995301ce90a0f63b2d5f9cbb30021a0f08f6ff))
* **nu:** Use `=` instead of space to pass command line parameters ([#3833](https://github.com/starship/starship/issues/3833)) ([2608db3](https://github.com/starship/starship/commit/2608db3a38b0dca13d91e94950fb4246b0ed1d82))
* **nu:** use shell-provided terminal width ([#3800](https://github.com/starship/starship/issues/3800)) ([859b780](https://github.com/starship/starship/commit/859b780b46780fdcac8141a9d165066880c36261))
* **nu:** use the most recent starship init ([#3908](https://github.com/starship/starship/issues/3908)) ([382445d](https://github.com/starship/starship/commit/382445dc4d21d190959f5582fb9b9febe056299a))
* **pwsh:** avoid potential deadlock in init ([#4335](https://github.com/starship/starship/issues/4335)) ([fd55397](https://github.com/starship/starship/commit/fd5539796f7a2b3750d1889b55a563d84b628bee))
* **pwsh:** fix error log display on older versions of pwsh ([#4650](https://github.com/starship/starship/issues/4650)) ([ef83e7a](https://github.com/starship/starship/commit/ef83e7a0928231b02650b3554ccd5bf21164aaff))
* regenerate changelog ([8a6be8c](https://github.com/starship/starship/commit/8a6be8c941de8e31330417bdb232204969a814ff))
* **rust:** overrides should only check full segments ([#3668](https://github.com/starship/starship/issues/3668)) ([076a9e6](https://github.com/starship/starship/commit/076a9e6b8e715fc200812f6a73a17a9764d45aba))
* **schema:** move config-schema into docs folder ([#3878](https://github.com/starship/starship/issues/3878)) ([094f982](https://github.com/starship/starship/commit/094f982df184eecd85ea2832b3bf638629118c10))
* some typos ([e7c1976](https://github.com/starship/starship/commit/e7c19765282eb31daf85e5eba26e13828bc2f6c7))
* **status:** Make status module work even when the status is 0 ([#3750](https://github.com/starship/starship/issues/3750)) ([8695327](https://github.com/starship/starship/commit/86953272a7f1471e9a3422a7543d97b953406df6))
* **status:** replace multiply with cross mark emoji ([#4461](https://github.com/starship/starship/issues/4461)) ([186d99e](https://github.com/starship/starship/commit/186d99e623d22fe9e2f7e52378f2ec4015f713d4))
* **timings:** count time spent on custom on 'when' command failure ([#4121](https://github.com/starship/starship/issues/4121)) ([aae1ed0](https://github.com/starship/starship/commit/aae1ed04babf4c7d8baaad670c076947d7200675))
* trigger another release ([81d2ce6](https://github.com/starship/starship/commit/81d2ce68ec05ea77151639267115d4daf73f5019))
* trigger release ([2cdf902](https://github.com/starship/starship/commit/2cdf902b57cd16dba42d4026e2e740537a82b0ee))
* Use git2::Repository::open_ext() instead of discover() ([#3591](https://github.com/starship/starship/issues/3591)) ([81a696a](https://github.com/starship/starship/commit/81a696a914f6761d42b69f139018c3fa663ff197))
* use shell-compatible curl install ([#3691](https://github.com/starship/starship/issues/3691)) ([48f1f75](https://github.com/starship/starship/commit/48f1f756f8922e2c24b1ee638bd0b8a7ae4be9aa))
* **windows:** avoid verbatim paths ([#3638](https://github.com/starship/starship/issues/3638)) ([1a8aa96](https://github.com/starship/starship/commit/1a8aa96b7fb488cf6306900eda417deb51188f99))


### Performance Improvements

* **directory:** Skip repo resolution if unused by directory config ([#4401](https://github.com/starship/starship/issues/4401)) ([227ec32](https://github.com/starship/starship/commit/227ec32d9d7e9d673360d487062fd4bef184e844))
* **git_commit:** only use exact match for tag by default ([#4281](https://github.com/starship/starship/issues/4281)) ([5984f08](https://github.com/starship/starship/commit/5984f0829ef5369e83c28108378fe0065a617b3c))
* **git_status:** add option to use windows starship to render in wsl ([#2146](https://github.com/starship/starship/issues/2146)) ([d2366dd](https://github.com/starship/starship/commit/d2366ddb9cf6d3ec288fc6aafd64edf2cef4d06d))
* **package:** only try to read files that exist ([#3904](https://github.com/starship/starship/issues/3904)) ([2a650bf](https://github.com/starship/starship/commit/2a650bfd140d561f955705cae124fb254ec549a1))
* **pulumi:** allow disabling upwards discovery ([#4159](https://github.com/starship/starship/issues/4159)) ([af15de9](https://github.com/starship/starship/commit/af15de93c4494bb08d8c2cb3dbf54951f6bc9239))
* **rayon:** restrict thread count ([#3667](https://github.com/starship/starship/issues/3667)) ([4369c92](https://github.com/starship/starship/commit/4369c92d4033c09ff411771e24c0161d713b7c64))
* **rust:** avoid calling `rustup` in more conditions ([#4174](https://github.com/starship/starship/issues/4174)) ([d8ac940](https://github.com/starship/starship/commit/d8ac940098eb16417742723c627d0de864597410))


### Reverts

* **schema:** move config-schema back into .github folder ([#3886](https://github.com/starship/starship/issues/3886)) ([9b2ce42](https://github.com/starship/starship/commit/9b2ce4240c602df368f966996d870ef9197e65ac))


### Miscellaneous Chores

* **master:** release 1.10.0 ([b974610](https://github.com/starship/starship/commit/b9746100e2ee3746ea418e26ebc50494710a9d03))
* **master:** release 1.9.1 ([6cbc362](https://github.com/starship/starship/commit/6cbc3620c9ee7695fe1dabc067b6fd339b8d2526))

## [1.12.0](https://github.com/starship/starship/compare/v1.11.0...v1.12.0) (2022-12-13)


### Features

* add Haxe support ([#4395](https://github.com/starship/starship/issues/4395)) ([2766c78](https://github.com/starship/starship/commit/2766c78749e638282d1dee56f7afcc195c16c064))
* Add operating system module ([#4109](https://github.com/starship/starship/issues/4109)) ([3109943](https://github.com/starship/starship/commit/3109943822a15b22faaa6cdfda17ca9554bcd800))
* **aws:** add a fallback for `expiration` key ([#4455](https://github.com/starship/starship/issues/4455)) ([5a2c85d](https://github.com/starship/starship/commit/5a2c85d078c1a8c83cc055dd0e56033abb15c2bf))
* **azure:** add username to azure module config ([#4323](https://github.com/starship/starship/issues/4323)) ([6e15c00](https://github.com/starship/starship/commit/6e15c00238a06e92cf411a669590002eb22324e7))
* **bug-report:** ask for confirmation before opening issue ([#4543](https://github.com/starship/starship/issues/4543)) ([8bb9038](https://github.com/starship/starship/commit/8bb9038431cd369e953ca156ed09aabd7c2ba326))
* **directory:** add before_repo_root_style ([#4595](https://github.com/starship/starship/issues/4595)) ([ea6249b](https://github.com/starship/starship/commit/ea6249b5243acf0cce2352a1b580479546b92340))
* **git_commit:** support showing lightweight tags ([#4632](https://github.com/starship/starship/issues/4632)) ([ac37792](https://github.com/starship/starship/commit/ac37792c19d7c545d4c51cf712f13e5e81559511))
* **guix_shell:** Initial implementation ([#4397](https://github.com/starship/starship/issues/4397)) ([d4bcc51](https://github.com/starship/starship/commit/d4bcc519e61524e1fe30f82412a09af113d75287))
* **init:** Use which-rs to resolve starship path ([cc2c8c4](https://github.com/starship/starship/commit/cc2c8c4a5450f2811612129abfbdc1aba12def91))
* **localip:** use reserved remote address ([#4648](https://github.com/starship/starship/issues/4648)) ([ddd54e9](https://github.com/starship/starship/commit/ddd54e9b20427b716e13d83884b4b0db03953210)), closes [#4614](https://github.com/starship/starship/issues/4614)
* **nu:** enable right prompt ([#4490](https://github.com/starship/starship/issues/4490)) ([a7abc0f](https://github.com/starship/starship/commit/a7abc0f4508b5357e44bc1d0a8b0ed363201824c)), closes [#3982](https://github.com/starship/starship/issues/3982)
* Open Policy Agent module ([#1740](https://github.com/starship/starship/issues/1740)) ([#4441](https://github.com/starship/starship/issues/4441)) ([865e68d](https://github.com/starship/starship/commit/865e68da3ad752a2bc85b923258f2dbd5287ada8))
* **package:** added showing gradle version based on the gradle.properties file ([#4432](https://github.com/starship/starship/issues/4432)) ([14ee81b](https://github.com/starship/starship/commit/14ee81b9c31047993217f060b57fb327a58c0d38))
* **preset:** Add No Empty Icons preset ([#4518](https://github.com/starship/starship/issues/4518)) ([1a3d51f](https://github.com/starship/starship/commit/1a3d51fe76c5a62d53533f5d14ceb4425d5a33a5))
* **preset:** Add no-nerd-font preset ([#4517](https://github.com/starship/starship/issues/4517)) ([4d86a4c](https://github.com/starship/starship/commit/4d86a4c7ae70dff552cdea85d7ea7872e2321c2f))
* **release:** add chocolatey publishing ([#4637](https://github.com/starship/starship/issues/4637)) ([df37e8d](https://github.com/starship/starship/commit/df37e8d40c7b3556f8428ce29c53f2882af2ce25))


### Bug Fixes

* **aws:** enable when using .aws/credentials ([#4604](https://github.com/starship/starship/issues/4604)) ([c8ac877](https://github.com/starship/starship/commit/c8ac8777a593358868813254c662da5fcb9fe6c8))
* **buf:** broken icon on windows 10 ([#4689](https://github.com/starship/starship/issues/4689)) ([7341607](https://github.com/starship/starship/commit/7341607c294a633477005d777bd03b18522aabf4))
* **ci:** cache after selecting the toolchain ([#4619](https://github.com/starship/starship/issues/4619)) ([e4dbff0](https://github.com/starship/starship/commit/e4dbff0fc7e88f792b90703f03f83e31d401b90e))
* **config:** unrecognized config properties don't cause config error ([#4547](https://github.com/starship/starship/issues/4547)) ([1b03ef2](https://github.com/starship/starship/commit/1b03ef21f34fc4acf890b01cfca3d07158ef5c46))
* **container:** avoid detecting WSL as a systemd-container ([#4593](https://github.com/starship/starship/issues/4593)) ([b47a4fe](https://github.com/starship/starship/commit/b47a4fe51470a36116b5c941c6e07ac5730585ea))
* don't attempt to display cmd_duration notification if in TTY ([#4535](https://github.com/starship/starship/issues/4535)) ([0427863](https://github.com/starship/starship/commit/04278631687da388005f2c26f3da2115b9075bf5))
* **git:** check `tag_disabled` option ([#4527](https://github.com/starship/starship/issues/4527)) ([fd165b9](https://github.com/starship/starship/commit/fd165b96cc9587f81ab68b580d371b71f4e0ff35))
* **java:** Improved regex for Java version (starship[#4610](https://github.com/starship/starship/issues/4610)) ([#4616](https://github.com/starship/starship/issues/4616)) ([a9eb65e](https://github.com/starship/starship/commit/a9eb65ef35de948880cbf340ffbfe6af126e5e44))
* **nu:** remove -c parameter from `term size` ([#4477](https://github.com/starship/starship/issues/4477)) ([4999530](https://github.com/starship/starship/commit/49995301ce90a0f63b2d5f9cbb30021a0f08f6ff))
* **pwsh:** fix error log display on older versions of pwsh ([#4650](https://github.com/starship/starship/issues/4650)) ([ef83e7a](https://github.com/starship/starship/commit/ef83e7a0928231b02650b3554ccd5bf21164aaff))
* **status:** replace multiply with cross mark emoji ([#4461](https://github.com/starship/starship/issues/4461)) ([186d99e](https://github.com/starship/starship/commit/186d99e623d22fe9e2f7e52378f2ec4015f713d4))

## [1.11.0](https://github.com/starship/starship/compare/v1.10.3...v1.11.0) (2022-10-14)


### Features

* add user-defined color palette ([#4209](https://github.com/starship/starship/issues/4209)) ([d93074d](https://github.com/starship/starship/commit/d93074d0569db4bafb1788aa3f39136b734b5370))
* **fish:** Enable left and right transience ([#4204](https://github.com/starship/starship/issues/4204)) ([06281c2](https://github.com/starship/starship/commit/06281c268d74a85d5b28e953bea251a2115f5568))
* **module:** Add a meson devenv indicator ([#4389](https://github.com/starship/starship/issues/4389)) ([355800f](https://github.com/starship/starship/commit/355800f8147b1755a5289dc679e2147abd662daf))
* **schema:** deny unknown keys ([#4270](https://github.com/starship/starship/issues/4270)) ([b5d3d8f](https://github.com/starship/starship/commit/b5d3d8fcf331cdff6d0e687dcdbac77351731475))
* **status:** Support formatting of pipestatus separator ([#4264](https://github.com/starship/starship/issues/4264)) ([6e35dfa](https://github.com/starship/starship/commit/6e35dfa85aeebb3f714389a9286623dc0f60d799))


### Bug Fixes

* **buf:** fix spacing & harmonize docs with actual configuration ([#4450](https://github.com/starship/starship/issues/4450)) ([3d45236](https://github.com/starship/starship/commit/3d452367bdde22a2554cc74bee4d1adfee7e8e04))
* **directory:** don't strip duplicate directory names twice ([#4295](https://github.com/starship/starship/issues/4295)) ([801fbab](https://github.com/starship/starship/commit/801fbab720e1bb94c32bb1aa10966a0637a10e63))
* **pwsh:** avoid potential deadlock in init ([#4335](https://github.com/starship/starship/issues/4335)) ([fd55397](https://github.com/starship/starship/commit/fd5539796f7a2b3750d1889b55a563d84b628bee))


### Performance Improvements

* **directory:** Skip repo resolution if unused by directory config ([#4401](https://github.com/starship/starship/issues/4401)) ([227ec32](https://github.com/starship/starship/commit/227ec32d9d7e9d673360d487062fd4bef184e844))

## [1.10.3](https://github.com/starship/starship/compare/v1.10.2...v1.10.3) (2022-09-07)


### Performance Improvements

* **git_commit:** only use exact match for tag by default ([#4281](https://github.com/starship/starship/issues/4281)) ([5984f08](https://github.com/starship/starship/commit/5984f0829ef5369e83c28108378fe0065a617b3c))

## [1.10.2](https://github.com/starship/starship/compare/v1.10.1...v1.10.2) (2022-08-18)


### Bug Fixes

* **git:** upgrade `gitoxide` to v0.21 ([#4277](https://github.com/starship/starship/issues/4277)) ([f52ae55](https://github.com/starship/starship/commit/f52ae552d3ef2c0c0c6de6429cee7b8271f14671))

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
