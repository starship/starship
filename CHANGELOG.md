# Changelog

## [1.10.0](https://github.com/starship/starship/compare/v1.10.0...v1.10.0) (2022-08-14)


### Features

* add a container indicator ([#3304](https://github.com/starship/starship/issues/3304)) ([4f46411](https://github.com/starship/starship/commit/4f46411403711a9ba0daa18353ecfe3a7a8720c6))
* Add a module for C projects ([#3631](https://github.com/starship/starship/issues/3631)) ([0863146](https://github.com/starship/starship/commit/0863146f072ae8382be63db26dcf9ddeff967aea))
* Add a Windows application manifest ([#3590](https://github.com/starship/starship/issues/3590)) ([a98908f](https://github.com/starship/starship/commit/a98908f05eab306c3e54820f153de4aa7df41cfe))
* add bun module ([#4187](https://github.com/starship/starship/issues/4187)) ([85692d1](https://github.com/starship/starship/commit/85692d1bf6a8477b6879adaf8b51007389df4328))
* Add package version detection for sbt projects ([#3274](https://github.com/starship/starship/issues/3274)) ([1109fd6](https://github.com/starship/starship/commit/1109fd69979f48cc6bce69d63d4e497727e51b8f))
* add Raku module ([#4048](https://github.com/starship/starship/issues/4048)) ([1a4fac6](https://github.com/starship/starship/commit/1a4fac63f78c9408756c19eb26af5181a7cf537e))
* Add starship preset command ([#4112](https://github.com/starship/starship/issues/4112)) ([c8a5adb](https://github.com/starship/starship/commit/c8a5adb412e98b07017ffa0edea5554b0a23b840))
* Add support for blink, hidden, and strikethrough styles. ([#4138](https://github.com/starship/starship/issues/4138)) ([aaab920](https://github.com/starship/starship/commit/aaab920f88015eb0a44e6514bf19b1db2b14829f))
* add support for cmd ([#3277](https://github.com/starship/starship/issues/3277)) ([c335b42](https://github.com/starship/starship/commit/c335b4267b80d584ed8a5d0dc7cfe7bf0bf7a74b))
* Add support for Daml ([#4004](https://github.com/starship/starship/issues/4004)) ([3fe6cc0](https://github.com/starship/starship/commit/3fe6cc023cd52917ae60a4d06ee6f1f78baa19e7))
* Add the ability to have some file extensions *prevent* a module from triggering ([#4043](https://github.com/starship/starship/issues/4043)) ([dd73447](https://github.com/starship/starship/commit/dd73447329e637ee207b1103ecb6a4bdbdc89324))
* add username to Pulumi module ([#3428](https://github.com/starship/starship/issues/3428)) ([568d057](https://github.com/starship/starship/commit/568d0570322ccc06f62237ea6b36d53e6a5bbf88))
* allow printing config file schema ([#3737](https://github.com/starship/starship/issues/3737)) ([18ad26f](https://github.com/starship/starship/commit/18ad26f98dd1bfcc01e2b092a5b6165a7b093631))
* **aws:** add option to force AWS display ([#3720](https://github.com/starship/starship/issues/3720)) ([e04f126](https://github.com/starship/starship/commit/e04f126a107eba2e40009f21942c14894385d6b0))
* **aws:** Add profile aliases ([#3699](https://github.com/starship/starship/issues/3699)) ([ac8c2fe](https://github.com/starship/starship/commit/ac8c2fe02474bee6fa41abf826501ec663cb0bb0))
* **azure:** Azure module ([#3275](https://github.com/starship/starship/issues/3275)) ([365b295](https://github.com/starship/starship/commit/365b295433638ce6ee32c15f2559d4b2d155e527))
* **buf:** Add Buf module ([#3661](https://github.com/starship/starship/issues/3661)) ([16f62d7](https://github.com/starship/starship/commit/16f62d790431ba2dd1fd02199a6924c00f6516d0))
* **cli:** Print arguments if argument parsing fails ([#3560](https://github.com/starship/starship/issues/3560)) ([c3cc40d](https://github.com/starship/starship/commit/c3cc40d2ac6f93b5e16177c274861b8f9c860e98))
* **cmd_duration:** Make notification timeout configurable ([#3515](https://github.com/starship/starship/issues/3515)) ([e680540](https://github.com/starship/starship/commit/e680540cfc4c783266183a589a26a33605bead43))
* **cmd_duration:** make notify feature optional (compat with nix darwin) ([#3855](https://github.com/starship/starship/issues/3855)) ([efaab49](https://github.com/starship/starship/commit/efaab49e4753bee1ce90ad08311a1d4dc04052b8))
* **deno:** detect `deno.json` and `deno.jsonc` ([#3220](https://github.com/starship/starship/issues/3220)) ([f48c7a2](https://github.com/starship/starship/commit/f48c7a26cfe6671be66d6e4e667e2069d5dafe84))
* **directory:** Windows path formatting via path_slash::PathBufExt ([#3157](https://github.com/starship/starship/issues/3157)) ([1c305c9](https://github.com/starship/starship/commit/1c305c9de7c7e02e60496833107cbff3fbda98c3))
* **elvish:** last command status ([#3403](https://github.com/starship/starship/issues/3403)) ([500dc3e](https://github.com/starship/starship/commit/500dc3ea6e2989a6a9e8b9960a8e9d7aa44d559d))
* Enable transience for Cmd and PowerShell ([#4143](https://github.com/starship/starship/issues/4143)) ([6e9c013](https://github.com/starship/starship/commit/6e9c013e60e59660cb7ae6289af5ed129ca85996))
* Enable Undistract Me for Linux ([#3547](https://github.com/starship/starship/issues/3547)) ([dcf6665](https://github.com/starship/starship/commit/dcf66659d0e87e619a1a881b2745eb68dbcd41a1))
* **git_branch:** add 'ignore_branches' option ([#3753](https://github.com/starship/starship/issues/3753)) ([bae16b5](https://github.com/starship/starship/commit/bae16b525de1f05a7ad125b5f4a8cb8baa7d5fae))
* **git:** replace `git2` with `git-repository` ([#3883](https://github.com/starship/starship/issues/3883)) ([ac55a01](https://github.com/starship/starship/commit/ac55a01d0ffe907ef7af48c9597c0bca4dbd8c69))
* **go:** check for go.work file to show Go module in prompt ([#3968](https://github.com/starship/starship/issues/3968)) ([9ebfce1](https://github.com/starship/starship/commit/9ebfce1e366656bd1c199bb50cc7e1bd6cdb90ad))
* **haskell:** Add Haskell module ([#3587](https://github.com/starship/starship/issues/3587)) ([72fec55](https://github.com/starship/starship/commit/72fec559c524247f4f92749dfd6702fcb3d97484))
* **hostname:** add `ssh_symbol` for ssh connections ([#3806](https://github.com/starship/starship/issues/3806)) ([2bf30dc](https://github.com/starship/starship/commit/2bf30dc89fbce6f4da37657b8af6077f15a543d0))
* **install:** Add posix shell check ([#3474](https://github.com/starship/starship/issues/3474)) ([a84a3af](https://github.com/starship/starship/commit/a84a3af0025d350eac69adf5170eba2699657a65))
* **k8s:** Add folder detection to the k8s module. ([#4157](https://github.com/starship/starship/issues/4157)) ([5c5969c](https://github.com/starship/starship/commit/5c5969c50b2490309b7ae9f7e6f5f75ea04a512d))
* **kubernetes:** add context user and cluster variables ([#3569](https://github.com/starship/starship/issues/3569)) ([d09f717](https://github.com/starship/starship/commit/d09f71720e1fada98ac02431685e200dd0847b96))
* **kubernetes:** add user alias ([#4008](https://github.com/starship/starship/issues/4008)) ([df5c2d8](https://github.com/starship/starship/commit/df5c2d8836622677460e34fa8082faa6b1a52835))
* **localip:** add module to print the current ipv4 address ([#3289](https://github.com/starship/starship/issues/3289)) ([5a26355](https://github.com/starship/starship/commit/5a26355b0e8211b42832eaaad205c8f2541abc20))
* **module:** Add `sudo` module ([#3135](https://github.com/starship/starship/issues/3135)) ([779e53c](https://github.com/starship/starship/commit/779e53cd66c56f13f55c4ad1367da1247cf44fdc))
* **nodejs:** check for `.mts` and `.cts` files ([#3734](https://github.com/starship/starship/issues/3734)) ([a10e24b](https://github.com/starship/starship/commit/a10e24b2052047d431b6a44b0a202f605c39bc96))
* **package:** Extract package version from PEP 621 compliant pyproject.toml ([#3950](https://github.com/starship/starship/issues/3950)) ([1b938fd](https://github.com/starship/starship/commit/1b938fd48420ceedf1e9886bd95ea738374680f7))
* **package:** support cargo workspace versions ([#4161](https://github.com/starship/starship/issues/4161)) ([0a1235e](https://github.com/starship/starship/commit/0a1235e27944f152ca195c32e7eef8985d475989))
* **package:** support for dart pub version ([#3373](https://github.com/starship/starship/issues/3373)) ([295948b](https://github.com/starship/starship/commit/295948bc6c2e8dbd463ab1bcdf12c4a28842693e))
* print-config subset of config ([#3179](https://github.com/starship/starship/issues/3179)) ([c3e33ea](https://github.com/starship/starship/commit/c3e33ea1c77e86cefabff09bfb7c55d10c9e541d))
* **pwsh:** Set ExtraPromptLineCount ([#3439](https://github.com/starship/starship/issues/3439)) ([0b184c3](https://github.com/starship/starship/commit/0b184c3ccbb9f97029642e139c604615eeb4ac95))
* **pwsh:** Set STARSHIP_SHELL to pwsh on PS >5 ([#3443](https://github.com/starship/starship/issues/3443)) ([7e32fd9](https://github.com/starship/starship/commit/7e32fd952ef0ef4b4765e22f5c2cfe827fff00ad))
* **python:** Show value of PYENV_VERSION when present ([#3144](https://github.com/starship/starship/issues/3144)) ([8d80d2e](https://github.com/starship/starship/commit/8d80d2ef062bdd9e8a7ae9d5ada7d465ae55c235))
* **release:** add windows msi installers ([#4031](https://github.com/starship/starship/issues/4031)) ([89fd532](https://github.com/starship/starship/commit/89fd5320af248207e8b253790bd191d8daa88dbe))
* **ruby:** Add environment variable checks to ruby module ([#3206](https://github.com/starship/starship/issues/3206)) ([d1ce352](https://github.com/starship/starship/commit/d1ce35252830c6b3a5329374aa2364177eafa583))
* **rust:** Display toolchain names ([#3414](https://github.com/starship/starship/issues/3414)) ([393d62c](https://github.com/starship/starship/commit/393d62c726573a0e6c351f422dbef4b17a5944bf))
* set a continuation prompt for supporting shells ([#3322](https://github.com/starship/starship/issues/3322)) ([4deaa02](https://github.com/starship/starship/commit/4deaa02d6fb3e72f286d822ac4c987b763c415dc))
* **spack:** Add `Spack` module ([#3639](https://github.com/starship/starship/issues/3639)) ([3014284](https://github.com/starship/starship/commit/3014284e952f75deae87973cbe2763b7a0a0aab5))
* starship bug-report sets syntax highlighting for config file ([#3529](https://github.com/starship/starship/issues/3529)) ([b99d3b8](https://github.com/starship/starship/commit/b99d3b8e24554f0cb308ce5bd779975735fe1741))
* **status:** Add hex_status ([#3312](https://github.com/starship/starship/issues/3312)) ([cb40787](https://github.com/starship/starship/commit/cb40787e2a5057bcc3a174545983d7efa011eda4))
* **status:** Add pipestatus_segment_format option to status module ([#4103](https://github.com/starship/starship/issues/4103)) ([6143848](https://github.com/starship/starship/commit/61438484bdc76601a185298f14337cfb4d5b4e0b))
* style git repo root ([#2010](https://github.com/starship/starship/issues/2010)) ([b07abc9](https://github.com/starship/starship/commit/b07abc990e640e4e2335e04b5b65b204fb2cbe88))
* **username:** Detect Admin access in Windows ([#2791](https://github.com/starship/starship/issues/2791)) ([c89c130](https://github.com/starship/starship/commit/c89c13038a34a52291d253e6d4b15c0dd4aa5dfa))
* **winget:** Add support for winget package manager ([#4042](https://github.com/starship/starship/issues/4042)) ([ef52f9e](https://github.com/starship/starship/commit/ef52f9e77ec66f5189a18acfdce399882c37fdd8))
* **xonsh:** support rprompt ([#3362](https://github.com/starship/starship/issues/3362)) ([2b40504](https://github.com/starship/starship/commit/2b405042b90ad88484fb1d90a822ed2f9494619e))


### Bug Fixes

* allow compilation without battery feature ([#3435](https://github.com/starship/starship/issues/3435)) ([8985499](https://github.com/starship/starship/commit/8985499c958cab961914286308f46e1622ceb038))
* **aws:** accept sso credentials ([#3718](https://github.com/starship/starship/issues/3718)) ([d730820](https://github.com/starship/starship/commit/d7308203a92bc067a3cb5177a5c6c32981c40959))
* **aws:** Make AWS_REGION orverrides AWS_DEFAULT_REGION ([#3619](https://github.com/starship/starship/issues/3619)) ([#3733](https://github.com/starship/starship/issues/3733)) ([59622bc](https://github.com/starship/starship/commit/59622bc41b5cf71e76d45b97681db298e8230656))
* **aws:** Only display AWS if there are credentials configured ([#3504](https://github.com/starship/starship/issues/3504)) ([e704549](https://github.com/starship/starship/commit/e70454956f24ce3b7727de81ac9b5fe26b7cc69f))
* **aws:** prevent endless loop when AWS config file is a directory ([#3335](https://github.com/starship/starship/issues/3335)) ([006fbf0](https://github.com/starship/starship/commit/006fbf0dd5b28f2d2f4f69a82c9a5a9a5344ac26))
* **aws:** support official `AWS_SHARED_CREDENTIALS_FILE` variable ([#4242](https://github.com/starship/starship/issues/4242)) ([1390036](https://github.com/starship/starship/commit/13900368826cf1aca160fd650f19cecc1a047372))
* **bash:** ensure `checkwinsize` is enabled for `$COLUMNS` ([#3832](https://github.com/starship/starship/issues/3832)) ([0334327](https://github.com/starship/starship/commit/03343272b778260016216266facd190936f9e7d3))
* **bash:** Restore previous exit status in bash init ([#3521](https://github.com/starship/starship/issues/3521)) ([6e24358](https://github.com/starship/starship/commit/6e24358052eea9267138225c81ff6f4986dcaadf))
* **bug-report:** remove git.io link shortening ([#3425](https://github.com/starship/starship/issues/3425)) ([673a198](https://github.com/starship/starship/commit/673a1981764e963667af2838134698fab3aece78))
* **character:** Standadise Vim config names ([#4081](https://github.com/starship/starship/issues/4081)) ([6761938](https://github.com/starship/starship/commit/67619386cdd7537f0ab9af77e701409e97a87917))
* **ci:** Version bump and fix Crowdin Pretranslate ([#3992](https://github.com/starship/starship/issues/3992)) ([a0a6c94](https://github.com/starship/starship/commit/a0a6c942fe3fc85d599aec883406224c9ecb589f))
* Correctly detect older versions of powershell in bug-report ([#3543](https://github.com/starship/starship/issues/3543)) ([5efb78b](https://github.com/starship/starship/commit/5efb78bcd3c5f28350b1af458a61bde53aaeb8a0))
* **dart:** detect version output in stdout with dart 2.15+ ([#3349](https://github.com/starship/starship/issues/3349)) ([8d0cebd](https://github.com/starship/starship/commit/8d0cebdcbdf7c4b771620523caa61f917c298b91))
* **directory:** enable repo_root_style when truncation_length is zero. ([#3536](https://github.com/starship/starship/issues/3536)) ([441ebb3](https://github.com/starship/starship/commit/441ebb39b9cd451564959d259409d2395e7afb01))
* **directory:** improve truncation detection ([#3266](https://github.com/starship/starship/issues/3266)) ([e18c61c](https://github.com/starship/starship/commit/e18c61cd684f77b13c65647065304f5fdc6a4080))
* Display durations of 0ms ([#3121](https://github.com/starship/starship/issues/3121)) ([a8579d6](https://github.com/starship/starship/commit/a8579d6f2feb3e4929de2b4b93f244479d1d6752))
* Do not panic in config if editor not found ([#3766](https://github.com/starship/starship/issues/3766)) ([2e80aec](https://github.com/starship/starship/commit/2e80aec5cb6f7376359e7a25a76a492a98717554))
* **docker_context:** ignore the "default" context ([#3803](https://github.com/starship/starship/issues/3803)) ([#3804](https://github.com/starship/starship/issues/3804)) ([230e85b](https://github.com/starship/starship/commit/230e85be37a0fc12999d1e6ff1209e7d5f99ecd1))
* **docs:** fix and cleanup VuePress config ([#3738](https://github.com/starship/starship/issues/3738)) ([7cdc230](https://github.com/starship/starship/commit/7cdc230100dc7208d9bb4957b0c01a65b7db60c0))
* **docs:** remove superfluous space ([#3314](https://github.com/starship/starship/issues/3314)) ([8d9650a](https://github.com/starship/starship/commit/8d9650afe21149ac18eead39ef5e25386d03a433))
* **elvish:** upgrade shell integration for v0.17 ([#3310](https://github.com/starship/starship/issues/3310)) ([67cddb6](https://github.com/starship/starship/commit/67cddb616bf375f882ea7118033e11a4daf81c6c))
* **elvish:** use `$pwd` for `logical-path` ([#3534](https://github.com/starship/starship/issues/3534)) ([6ca911b](https://github.com/starship/starship/commit/6ca911b9fe1c9f8eae6ebb28f55e81106379625d))
* escape text segments in meta variables ([#3563](https://github.com/starship/starship/issues/3563)) ([7d31bac](https://github.com/starship/starship/commit/7d31bac1cc3f39bd02f2e188e69283c566b816ed))
* **escaping:** move escaping to individual variables ([#3107](https://github.com/starship/starship/issues/3107)) ([c1f2d34](https://github.com/starship/starship/commit/c1f2d345aac0b0241ea1b6d99977fea20fa3f5bb))
* Exclude vuepress output from dprint ([#3616](https://github.com/starship/starship/issues/3616)) ([532efaa](https://github.com/starship/starship/commit/532efaadfe0745913172e97d4b7be4bbaab38a2e))
* **fish:** add proper vi mode detection for fish shell ([#3839](https://github.com/starship/starship/issues/3839)) ([1469763](https://github.com/starship/starship/commit/146976351ec804ab1594d5262a1e0dd2d2de4972))
* **fish:** allow generating session keys in older versions of fish ([#3697](https://github.com/starship/starship/issues/3697)) ([0fb4219](https://github.com/starship/starship/commit/0fb421969058ec07a09f7c927dddc1258de75631))
* **fish:** Emit clear-screen escape sequence only in left prompt ([#3588](https://github.com/starship/starship/issues/3588)) ([e9e090e](https://github.com/starship/starship/commit/e9e090e97eef3e6b9c74e0f1e183772cc2fa9b1d))
* fix release-please permissions ([23be606](https://github.com/starship/starship/commit/23be606516b5815fafea211aa59d2315ddb77df7))
* **git_branch:** correct variable name for remote branch ([#3897](https://github.com/starship/starship/issues/3897)) ([bd7957f](https://github.com/starship/starship/commit/bd7957f01c7fa2b14f068e4130f1aedea61f4a76))
* **git_branch:** more robust handling of .git ([#3290](https://github.com/starship/starship/issues/3290)) ([e3a88a6](https://github.com/starship/starship/commit/e3a88a6ec1bf09ad87a5a56e389da6c9c4f96f2a))
* ignore empty `--jobs` argument ([#3593](https://github.com/starship/starship/issues/3593)) ([0ea16e2](https://github.com/starship/starship/commit/0ea16e2641b39450071b1f22efeed526e9420932))
* ignore scan_dir timeout in tests ([#3184](https://github.com/starship/starship/issues/3184)) ([6e6ab9f](https://github.com/starship/starship/commit/6e6ab9f212ef9c1c6984b6250ec867c864ca378b))
* init script line endings ([#3178](https://github.com/starship/starship/issues/3178)) ([265b92b](https://github.com/starship/starship/commit/265b92bd51a3a3d526353cf29b2c44b694ace584))
* **init:** Change Elvish init to `catch` for 0.18 ([#3769](https://github.com/starship/starship/issues/3769)) ([538329d](https://github.com/starship/starship/commit/538329d9b406cd6358d0fe31d58e0c9f578ceffa))
* **init:** Change Nushell init for nu 0.60 ([#3773](https://github.com/starship/starship/issues/3773)) ([c9b75fe](https://github.com/starship/starship/commit/c9b75fe115075c23eb456df5b1af8f4491834aaf))
* **install:** Add -o flag to unzip to match tar ([#3727](https://github.com/starship/starship/issues/3727)) ([ef96727](https://github.com/starship/starship/commit/ef967271e6009d3515fdd4c3dd64f575676411a9))
* **install:** Have fixed a spacing issue in output  ([#4082](https://github.com/starship/starship/issues/4082)) ([2ffe173](https://github.com/starship/starship/commit/2ffe1737f06db4ce89a21b2b5238f3ad76c94bca))
* **install:** ignore tarfile ownership values when installing as root ([#4046](https://github.com/starship/starship/issues/4046)) ([1a91510](https://github.com/starship/starship/commit/1a91510beda1de2c3b149b7aacc0d76cf4652482))
* **localip:** disable localip module default ([#3607](https://github.com/starship/starship/issues/3607)) ([efb16dd](https://github.com/starship/starship/commit/efb16dd9cabc04ce0c5c34e5ea67c50c3a63c433))
* **memory_usage:** remove duplicate `%` handling ([#3193](https://github.com/starship/starship/issues/3193)) ([4ee1bdc](https://github.com/starship/starship/commit/4ee1bdc2a4ad4b54924db4b854d8c2a6ab4431d1))
* **module:** list option not working ([#3919](https://github.com/starship/starship/issues/3919)) ([6fe6735](https://github.com/starship/starship/commit/6fe6735927170b9f2aaa10cb84fa3a4d754e3bd6))
* **nodejs:** use e718 as the default of symbol in node configuration ([#3533](https://github.com/starship/starship/issues/3533)) ([65f2975](https://github.com/starship/starship/commit/65f29754d3a97a16ff4372bb59397f711787e54a))
* **nu:** don't use `cygpath` for starship binary path in init ([#4001](https://github.com/starship/starship/issues/4001)) ([9b52475](https://github.com/starship/starship/commit/9b52475e541f751e8c650587cd8c1615fe00b1d0))
* **nu:** Use `=` instead of space to pass command line parameters ([#3833](https://github.com/starship/starship/issues/3833)) ([2608db3](https://github.com/starship/starship/commit/2608db3a38b0dca13d91e94950fb4246b0ed1d82))
* **nu:** use shell-provided terminal width ([#3800](https://github.com/starship/starship/issues/3800)) ([859b780](https://github.com/starship/starship/commit/859b780b46780fdcac8141a9d165066880c36261))
* **nu:** use the most recent starship init ([#3908](https://github.com/starship/starship/issues/3908)) ([382445d](https://github.com/starship/starship/commit/382445dc4d21d190959f5582fb9b9febe056299a))
* only print root level config logs a single time ([#3132](https://github.com/starship/starship/issues/3132)) ([c443953](https://github.com/starship/starship/commit/c4439531d35dfbed008a0dd519aa89bb67dcce24))
* **properties:** fix regressions in `status`, `pipestatus` and `terminal-width` handling ([#3399](https://github.com/starship/starship/issues/3399)) ([0fd6f05](https://github.com/starship/starship/commit/0fd6f05da430c38e420642ff90963470312eef69))
* **pwsh:** Avoid polluting the global function namespace ([#3424](https://github.com/starship/starship/issues/3424)) ([0b6ffca](https://github.com/starship/starship/commit/0b6ffca35d7a129bb629a192b51cdfc637b6a4a8))
* **pwsh:** Use global:error[0] for most recent error in powershell ([#3584](https://github.com/starship/starship/issues/3584)) ([465e6fc](https://github.com/starship/starship/commit/465e6fc4be64bf830d2c98bc3f56c2601acef775))
* regenerate changelog ([8a6be8c](https://github.com/starship/starship/commit/8a6be8c941de8e31330417bdb232204969a814ff))
* replace battery with starship-battery ([#3213](https://github.com/starship/starship/issues/3213)) ([9df7c7d](https://github.com/starship/starship/commit/9df7c7d256a1a6da296b40a233d44ee86d48d431))
* **rust:** fix overrides on windows and set cwd ([#3359](https://github.com/starship/starship/issues/3359)) ([3b7446f](https://github.com/starship/starship/commit/3b7446fdf38ef80f0eebce3d1382b916167f1e9c))
* **rust:** overrides should only check full segments ([#3668](https://github.com/starship/starship/issues/3668)) ([076a9e6](https://github.com/starship/starship/commit/076a9e6b8e715fc200812f6a73a17a9764d45aba))
* save pipestatus in fish init functions ([#3160](https://github.com/starship/starship/issues/3160)) ([8ae6548](https://github.com/starship/starship/commit/8ae6548dc06aaae383e17f7f97ab38c3a572df40))
* **schema:** move config-schema into docs folder ([#3878](https://github.com/starship/starship/issues/3878)) ([094f982](https://github.com/starship/starship/commit/094f982df184eecd85ea2832b3bf638629118c10))
* set cwd for command execution ([#3309](https://github.com/starship/starship/issues/3309)) ([af98f5b](https://github.com/starship/starship/commit/af98f5b8ceadb1cfbd97da8777e4cfdf4822da5d))
* some typos ([e7c1976](https://github.com/starship/starship/commit/e7c19765282eb31daf85e5eba26e13828bc2f6c7))
* **status:** Enable to convert from i64 to hex_status by casting instead of parsing status. ([#3462](https://github.com/starship/starship/issues/3462)) ([bbdb584](https://github.com/starship/starship/commit/bbdb584f45ddfe20a7f9b3aef665ce322f7db056))
* **status:** Make status module work even when the status is 0 ([#3750](https://github.com/starship/starship/issues/3750)) ([8695327](https://github.com/starship/starship/commit/86953272a7f1471e9a3422a7543d97b953406df6))
* Store $? and $LASTEXITCODE first in PowerShell ([#3316](https://github.com/starship/starship/issues/3316)) ([b21904c](https://github.com/starship/starship/commit/b21904c9d3ac8ab8504badc5de5e1bf32484cbef)), closes [#3315](https://github.com/starship/starship/issues/3315)
* **timings:** count time spent on custom on 'when' command failure ([#4121](https://github.com/starship/starship/issues/4121)) ([aae1ed0](https://github.com/starship/starship/commit/aae1ed04babf4c7d8baaad670c076947d7200675))
* trigger another release ([81d2ce6](https://github.com/starship/starship/commit/81d2ce68ec05ea77151639267115d4daf73f5019))
* trigger release ([2cdf902](https://github.com/starship/starship/commit/2cdf902b57cd16dba42d4026e2e740537a82b0ee))
* typo in FAQ page ([#3347](https://github.com/starship/starship/issues/3347)) ([fffc756](https://github.com/starship/starship/commit/fffc7561f67057be4df952da2bc0f7cd41b28b33))
* typo of variable in CONTRIBUTING ([#3595](https://github.com/starship/starship/issues/3595)) ([7347d2c](https://github.com/starship/starship/commit/7347d2c195c0ea5bd8f940328f8815e15867a28c))
* update continuation prompt to be more minimal ([#3374](https://github.com/starship/starship/issues/3374)) ([800fbec](https://github.com/starship/starship/commit/800fbec0cfeb7d5a2ff6b2dd57fcf3b080d8ec99))
* Use git2::Repository::open_ext() instead of discover() ([#3591](https://github.com/starship/starship/issues/3591)) ([81a696a](https://github.com/starship/starship/commit/81a696a914f6761d42b69f139018c3fa663ff197))
* use shell-compatible curl install ([#3691](https://github.com/starship/starship/issues/3691)) ([48f1f75](https://github.com/starship/starship/commit/48f1f756f8922e2c24b1ee638bd0b8a7ae4be9aa))
* **windows:** avoid verbatim paths ([#3638](https://github.com/starship/starship/issues/3638)) ([1a8aa96](https://github.com/starship/starship/commit/1a8aa96b7fb488cf6306900eda417deb51188f99))


### Performance Improvements

* **elvish:** Use built-in `randint` instead of `starship session`. ([#3479](https://github.com/starship/starship/issues/3479)) ([0d573ac](https://github.com/starship/starship/commit/0d573ac5ea5063a0a488879be92f912fcb5b541c))
* **git_status:** add option to use windows starship to render in wsl ([#2146](https://github.com/starship/starship/issues/2146)) ([d2366dd](https://github.com/starship/starship/commit/d2366ddb9cf6d3ec288fc6aafd64edf2cef4d06d))
* **git_status:** tweak exec flags to omit unnecessary info ([#3287](https://github.com/starship/starship/issues/3287)) ([a953324](https://github.com/starship/starship/commit/a95332485b690c9147c3265f272898ce503ad643))
* **package:** only try to read files that exist ([#3904](https://github.com/starship/starship/issues/3904)) ([2a650bf](https://github.com/starship/starship/commit/2a650bfd140d561f955705cae124fb254ec549a1))
* **pulumi:** allow disabling upwards discovery ([#4159](https://github.com/starship/starship/issues/4159)) ([af15de9](https://github.com/starship/starship/commit/af15de93c4494bb08d8c2cb3dbf54951f6bc9239))
* **rayon:** restrict thread count ([#3667](https://github.com/starship/starship/issues/3667)) ([4369c92](https://github.com/starship/starship/commit/4369c92d4033c09ff411771e24c0161d713b7c64))
* **rust:** additionally check `rustup default` for faster result. ([#3354](https://github.com/starship/starship/issues/3354)) ([c63e9a7](https://github.com/starship/starship/commit/c63e9a71bd958c576100fbbeaf5723bb22450fd9))
* **rust:** avoid calling `rustup` in more conditions ([#4174](https://github.com/starship/starship/issues/4174)) ([d8ac940](https://github.com/starship/starship/commit/d8ac940098eb16417742723c627d0de864597410))


### Reverts

* **schema:** move config-schema back into .github folder ([#3886](https://github.com/starship/starship/issues/3886)) ([9b2ce42](https://github.com/starship/starship/commit/9b2ce4240c602df368f966996d870ef9197e65ac))


### Miscellaneous Chores

* **master:** release 1.10.0 ([b974610](https://github.com/starship/starship/commit/b9746100e2ee3746ea418e26ebc50494710a9d03))
* **master:** release 1.9.1 ([6cbc362](https://github.com/starship/starship/commit/6cbc3620c9ee7695fe1dabc067b6fd339b8d2526))

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
