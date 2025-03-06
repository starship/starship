# Changelog

## [1.22.1](https://github.com/starship/starship/compare/v1.22.0...v1.22.1) (2025-01-11)


### Bug Fixes

* **windows:** downgrade libz-ng-sys ([#6489](https://github.com/starship/starship/issues/6489)) ([6f7c67b](https://github.com/starship/starship/commit/6f7c67b4858b755c19754a2ec1063502545a16b6))

## [1.22.0](https://github.com/starship/starship/compare/v1.21.1...v1.22.0) (2025-01-11)


### Features

* **battery:** Use best match instead of first match for `battery.display` threshold ([#6442](https://github.com/starship/starship/issues/6442)) ([2690a32](https://github.com/starship/starship/commit/2690a329cf9a6b4a160d024674193007fb292e1a))
* **bun:** Detect bun using new text-format lockfile ([#6441](https://github.com/starship/starship/issues/6441)) ([2df521c](https://github.com/starship/starship/commit/2df521c69b0b1400edee7b7b332b5e2a900c7775))
* **direnv:** allow direnv to detect environment variables ([#6196](https://github.com/starship/starship/issues/6196)) ([4593716](https://github.com/starship/starship/commit/45937166c8b3cb53f09b29c65d0f547aa7f2f022))
* **status:** add `{success,failure}_style` options ([#6269](https://github.com/starship/starship/issues/6269)) ([e21775c](https://github.com/starship/starship/commit/e21775c694a81cf092eafa3c9a7b169ae9c8ebf2))


### Bug Fixes

* **fish:** restore support for fish &lt;v3.4.0 ([#6337](https://github.com/starship/starship/issues/6337)) ([85d3fb2](https://github.com/starship/starship/commit/85d3fb2e2ab96e639fe6b67ba1afbb94b6c5b83e))
* handle variable bash $SHLVL behavior with explicit option ([#4912](https://github.com/starship/starship/issues/4912)) ([7ead2b5](https://github.com/starship/starship/commit/7ead2b55eb58a878dc3ea8d87204f84862bd8148))
* **hostname:** use DNS instead of NetBIOS name on Windows ([#6343](https://github.com/starship/starship/issues/6343)) ([43f097e](https://github.com/starship/starship/commit/43f097eae838c1fba52659113826669802081918))
* **init/pwsh:** preserve ViModeChangeHandler ([#6225](https://github.com/starship/starship/issues/6225)) ([9b6d394](https://github.com/starship/starship/commit/9b6d394e01d50db4395cfe402531bfd96c540462))
* **init:** do not use cygpath for elvish ([#6376](https://github.com/starship/starship/issues/6376)) ([576aa14](https://github.com/starship/starship/commit/576aa14d15f57abf7d1c28e3486f2682e45c2f62))
* **status:** add trailing space to `pipestatus_format` ([#6404](https://github.com/starship/starship/issues/6404)) ([8a38b8f](https://github.com/starship/starship/commit/8a38b8f6f19fa6f17098e32f22183beac0498764))
* **username:** avoid using `whoami` on android ([#6342](https://github.com/starship/starship/issues/6342)) ([af5d139](https://github.com/starship/starship/commit/af5d139060bbb60690196dc53e05424ac3c0fb5d)), closes [#6340](https://github.com/starship/starship/issues/6340)

## [1.21.1](https://github.com/starship/starship/compare/v1.21.0...v1.21.1) (2024-10-18)


### Reverts

* "perf(fish): Skip unnecessary indirection in starship init fish" ([#6326](https://github.com/starship/starship/issues/6326)) ([00d927b](https://github.com/starship/starship/commit/00d927bb20e618b587635876d2e10047f98eff88))

## [1.21.0](https://github.com/starship/starship/compare/v1.20.1...v1.21.0) (2024-10-17)


### Features

* **deno:** add `deno.lock` file to default `detect_files` list ([#6310](https://github.com/starship/starship/issues/6310)) ([22c6c52](https://github.com/starship/starship/commit/22c6c5201d8fa682ad43501cf46427263a530c71))
* **hostname:** add option to replace hostnames with aliases ([#6097](https://github.com/starship/starship/issues/6097)) ([68a8fc9](https://github.com/starship/starship/commit/68a8fc9563a52c4119241375d181ff68bb3d3da3))
* **mojo:** add module for Mojo language ([#6108](https://github.com/starship/starship/issues/6108)) ([171746a](https://github.com/starship/starship/commit/171746a58408cbc0d48dedc52e21327bd35fccd4))
* **package:** support alternative gradle module version syntax ([#6079](https://github.com/starship/starship/issues/6079)) ([a62012e](https://github.com/starship/starship/commit/a62012e3280238b4c794bbe484b296af3b8fb53a))
* **python:** Add detect env vars option  ([#4486](https://github.com/starship/starship/issues/4486)) ([8a2d944](https://github.com/starship/starship/commit/8a2d944bc80a0a8358c30e639e8af51c1d45d973))
* **python:** detect pixi and ipynb files ([#6228](https://github.com/starship/starship/issues/6228)) ([36134d8](https://github.com/starship/starship/commit/36134d896bda3c739ffd6555be62499716b0a9b0))
* **release:** codesign windows release binaries ([#6273](https://github.com/starship/starship/issues/6273)) ([fcc697b](https://github.com/starship/starship/commit/fcc697b9b31aef44aefaf9a6660d51d44f707149))


### Bug Fixes

* **bash:** fix variable leak in Bash integration ([#6143](https://github.com/starship/starship/issues/6143)) ([e1189ed](https://github.com/starship/starship/commit/e1189ed7567de580d8f51c8baf870056a6060789))
* **cmd_duration:** Make render_time format more consistent ([#5825](https://github.com/starship/starship/issues/5825)) ([4abea6b](https://github.com/starship/starship/commit/4abea6b601c53adfa5719a7aac11069dc054cdf8))
* **docker_context:** Ignore Docker Desktop "desktop-linux" context. ([#6314](https://github.com/starship/starship/issues/6314)) ([14babde](https://github.com/starship/starship/commit/14babde435f6307a8a28f9d323f0d66a319e9eab))
* **docs:** correct username options ([#6084](https://github.com/starship/starship/issues/6084)) ([d96fbc5](https://github.com/starship/starship/commit/d96fbc5bcb9c039f875c4a55cff31edda57d5e6e))
* **fish:** add missing arguments for fish transient prompt functions ([#6181](https://github.com/starship/starship/issues/6181)) ([d6814be](https://github.com/starship/starship/commit/d6814be0ba35dd52f6f0d0e1a3f14d5f106efd63)), closes [#6180](https://github.com/starship/starship/issues/6180)
* **fish:** improve fish transient prompt ([#6107](https://github.com/starship/starship/issues/6107)) ([274dff3](https://github.com/starship/starship/commit/274dff37dbb375821a90d6976a11cf57d9386e4f))
* **fish:** use correct input function in transient execute ([#6265](https://github.com/starship/starship/issues/6265)) ([244c2c6](https://github.com/starship/starship/commit/244c2c629e1f358bf849923531d5cf0d69c2db2d))
* Fixed "Click to download TOML" links ([#6234](https://github.com/starship/starship/issues/6234)) ([0200d75](https://github.com/starship/starship/commit/0200d75b3781977d5335e637858f4e0996ee0384))
* **git_status:** read proper name for core.fsmonitor flag ([#6173](https://github.com/starship/starship/issues/6173)) ([33b3b8e](https://github.com/starship/starship/commit/33b3b8e3986f6180a8f7b461a5a07fea702ec628))
* **package:** support alternative gradle module version syntax ([#6080](https://github.com/starship/starship/issues/6080)) ([a62012e](https://github.com/starship/starship/commit/a62012e3280238b4c794bbe484b296af3b8fb53a))
* **python:** improve parsing of `pyvenv.cfg` files ([#6145](https://github.com/starship/starship/issues/6145)) ([7b65ad5](https://github.com/starship/starship/commit/7b65ad51635463003941ec0ecea1d4d315b2df53))
* use `whoami` for user/hostname queries again ([#6311](https://github.com/starship/starship/issues/6311)) ([77f0883](https://github.com/starship/starship/commit/77f08832d5228dff6c0e01ba4d4ad206c9d2362f))


### Performance Improvements

* **fish:** Skip unnecessary indirection in starship init fish ([#6253](https://github.com/starship/starship/issues/6253)) ([798f640](https://github.com/starship/starship/commit/798f64033f5348793003ddd74fa0ce1c744c300b))
* Skip unnecessary indirection in starship init fish ([798f640](https://github.com/starship/starship/commit/798f64033f5348793003ddd74fa0ce1c744c300b))

## [1.20.1](https://github.com/starship/starship/compare/v1.20.0...v1.20.1) (2024-07-26)


### Bug Fixes

* **ci:** move to macos-latest image ([#6132](https://github.com/starship/starship/issues/6132)) ([c7a8ea7](https://github.com/starship/starship/commit/c7a8ea757033512b5723a39be71bdb91fdcfbdb1))

## [1.20.0](https://github.com/starship/starship/compare/v1.19.0...v1.20.0) (2024-07-26)


### Features

* **color:** add prev_fg and prev_bg as color specifiers based on the previous foreground/background colors respectively ([#6017](https://github.com/starship/starship/issues/6017)) ([9a3e87f](https://github.com/starship/starship/commit/9a3e87f2cbaa8ca4b8dea41f4f56ff297e94bb1a))
* **color:** add prevfg,prevbg as color specifiers based on the previous foreground/background colors respectively ([9a3e87f](https://github.com/starship/starship/commit/9a3e87f2cbaa8ca4b8dea41f4f56ff297e94bb1a))
* **gcloud:** Disabled gcloud module when active config hasn't been set ([#6039](https://github.com/starship/starship/issues/6039)) ([4d36d46](https://github.com/starship/starship/commit/4d36d463b91a245a62c357de308cad3cdec48edf))
* **purescript:** add support for spago-next configuration files ([#6036](https://github.com/starship/starship/issues/6036)) ([e028186](https://github.com/starship/starship/commit/e0281868c9d7801fc25f64f86ab8312d2a5a1794))


### Bug Fixes

* **direnv:** replace nonexistent "orange" color with "bright-yellow" ([#5969](https://github.com/starship/starship/issues/5969)) ([478d5cc](https://github.com/starship/starship/commit/478d5cccb6f7f4622dd681756cf004ba92c98607))
* **dotnet:** Remove duplicate v in dotnet version  ([#6060](https://github.com/starship/starship/issues/6060)) ([52a89a5](https://github.com/starship/starship/commit/52a89a515ed44d861ea556a0bb70d6901660a9b5))
* improve non-msvc windows support by bumping stable dependencies ([#5989](https://github.com/starship/starship/issues/5989)) ([50a6b84](https://github.com/starship/starship/commit/50a6b846045899c88d95b063a6b939a6dcb72a5c))
* **k8s:** don't trigger if scan config is set but env vars are not ([#5979](https://github.com/starship/starship/issues/5979)) ([8c75822](https://github.com/starship/starship/commit/8c75822d4380e020ebd289a4ef06cd4d31ee5196))
* **preset:** fix typo in Open Policy Agent module format string ([#6118](https://github.com/starship/starship/issues/6118)) ([eba9748](https://github.com/starship/starship/commit/eba9748eb1bd9e1802d2a770eb0907cdb73292d6))


### Performance Improvements

* **k8s:** Improve performance of kubeconfig module ([#6032](https://github.com/starship/starship/issues/6032)) ([fae92b2](https://github.com/starship/starship/commit/fae92b29646f73bf1eb843fde80885c58480e85b))

## [1.19.0](https://github.com/starship/starship/compare/v1.18.2...v1.19.0) (2024-05-15)


### Features

* **container:** show name if available ([#5195](https://github.com/starship/starship/issues/5195)) ([3e3f18e](https://github.com/starship/starship/commit/3e3f18ef277c9bb3db9f60c964f280c7d981102c))
* **gleam:** Add Gleam module ([#5886](https://github.com/starship/starship/issues/5886)) ([34a8f7e](https://github.com/starship/starship/commit/34a8f7e62845fd66df3f6f9003cb850f2b0d9bc5))
* **k8s:** Add detect env vars option ([#4488](https://github.com/starship/starship/issues/4488)) ([e3b5dff](https://github.com/starship/starship/commit/e3b5dff3524608328de6ee3cbb390f8c640b0ce0))
* **nats:** Add NATS Context ([#5900](https://github.com/starship/starship/issues/5900)) ([5facd85](https://github.com/starship/starship/commit/5facd85da56d28e74878286923f148bc468f71c3))
* **odin:** Add Odin Lang module ([#5873](https://github.com/starship/starship/issues/5873)) ([335c514](https://github.com/starship/starship/commit/335c514e9ee5fa5bc8a8294b4804ddf198504b97))
* **username:** add aliases option ([#5855](https://github.com/starship/starship/issues/5855)) ([9c1eadd](https://github.com/starship/starship/commit/9c1eaddae154daed9e1d00faac414586f7920013))


### Bug Fixes

* **bash:** fix handling of the preserved DEBUG trap (used in Bash &lt;= 4.3) ([#5908](https://github.com/starship/starship/issues/5908)) ([fe1a8df](https://github.com/starship/starship/commit/fe1a8df6fa5121cc83b288cf054a8ed16631d335))
* **bash:** use `eval` instead of a procsub for the POSIX mode ([#5020](https://github.com/starship/starship/issues/5020)) ([0f859e8](https://github.com/starship/starship/commit/0f859e8b61c8ada354f4ea4147fb8752dca02c51))
* **bash:** use eval instead of a procsub for the POSIX mode ([0f859e8](https://github.com/starship/starship/commit/0f859e8b61c8ada354f4ea4147fb8752dca02c51))
* **bun:** change default icon ([#5880](https://github.com/starship/starship/issues/5880)) ([be4e97c](https://github.com/starship/starship/commit/be4e97c2d9169cbf0bc30e271dd4ba07530dc46f))
* **scala:** use scala-cli first to avoid timeout ([2221a88](https://github.com/starship/starship/commit/2221a88f5ada124c657a265b878e37be47998766))
* **xonsh:** remove `| cat` workaround from xonsh init ([#5113](https://github.com/starship/starship/issues/5113)) ([de5b7f0](https://github.com/starship/starship/commit/de5b7f041a8c88ecf912b3be3b32d70f76518348))


### Performance Improvements

* **scala:** use scala-cli first to avoid timeout ([#5952](https://github.com/starship/starship/issues/5952)) ([2221a88](https://github.com/starship/starship/commit/2221a88f5ada124c657a265b878e37be47998766))

## [1.18.2](https://github.com/starship/starship/compare/v1.18.1...v1.18.2) (2024-03-29)


### Bug Fixes

* replace unmaintained crates `yaml-rust`, `dirs-next` ([#5887](https://github.com/starship/starship/issues/5887)) ([796a411](https://github.com/starship/starship/commit/796a411602c9ca4e5103c54247440f4efe892918))


### Reverts

* "build(deps): update rust crate gix to 0.61.1" ([#5878](https://github.com/starship/starship/issues/5878)) ([eb80dba](https://github.com/starship/starship/commit/eb80dbab99e38b5018aa3fb70b06ae9e4d793b24))

## [1.18.1](https://github.com/starship/starship/compare/v1.18.0...v1.18.1) (2024-03-24)


### Bug Fixes

* **deps:** update rust crate os_info to 3.8.2 ([#5870](https://github.com/starship/starship/issues/5870)) ([d421f63](https://github.com/starship/starship/commit/d421f63785f901caf222941cbfae7cb8c0feec74))
* replace all remaining paths referring to vuepress ([#5859](https://github.com/starship/starship/issues/5859)) ([d5861f9](https://github.com/starship/starship/commit/d5861f9f974147d307c319865890cbb3e50dbe5a))

## [1.18.0](https://github.com/starship/starship/compare/v1.17.1...v1.18.0) (2024-03-21)


### Features

* $gemset variable for Ruby module ([#5429](https://github.com/starship/starship/issues/5429)) ([938ea3c](https://github.com/starship/starship/commit/938ea3c40145af68d8e28b2ef0275531f1123202))
* **bash:** Support right prompt and transience ([#4902](https://github.com/starship/starship/issues/4902)) ([5ead13d](https://github.com/starship/starship/commit/5ead13d6aa6303c85c562f1b940048cc539667cd))
* **bash:** use PS0 for preexec hook ([#5735](https://github.com/starship/starship/issues/5735)) ([ae711c0](https://github.com/starship/starship/commit/ae711c0f332f4f24c843e59d6d5783e398e21b38))
* **direnv:** use JSON status with direnv &gt;= 2.33.0 ([#5692](https://github.com/starship/starship/issues/5692)) ([482c7b7](https://github.com/starship/starship/commit/482c7b719fc304fcad5f3572c4551f8ff4179522))
* **docs:** move to vitepress ([#5785](https://github.com/starship/starship/issues/5785)) ([7485c90](https://github.com/starship/starship/commit/7485c90c9f7259c026a84dd0335f56860005315d))
* **install:** Add version option to install script ([f66bfd9](https://github.com/starship/starship/commit/f66bfd9435f215867681a699428bd882d8c63ce7))
* **install:** Add version option to install script ([#5728](https://github.com/starship/starship/issues/5728)) ([f66bfd9](https://github.com/starship/starship/commit/f66bfd9435f215867681a699428bd882d8c63ce7))
* **os:** add new os symbols ([#5849](https://github.com/starship/starship/issues/5849)) ([df65b21](https://github.com/starship/starship/commit/df65b2155f92c03d07cabbcee6c92104a878c963))
* **quarto:** Add Quarto module ([#5820](https://github.com/starship/starship/issues/5820)) ([0e49f04](https://github.com/starship/starship/commit/0e49f04a6b249090cf3703c5dac041a51e6bf530))
* **release:** add winget arm64 push & repo sync ([#5033](https://github.com/starship/starship/issues/5033)) ([aef1a3f](https://github.com/starship/starship/commit/aef1a3f275e7ec9095c4718ecf6a268abfe7794a))
* **username:** add detect_env_vars as option ([#5833](https://github.com/starship/starship/issues/5833)) ([b8a812b](https://github.com/starship/starship/commit/b8a812b93207da992ff8ae9f8b955bfa1252072b))


### Bug Fixes

* **bash:** Handle Unbound Variables Errors in Bash ([#4972](https://github.com/starship/starship/issues/4972)) ([7093d5c](https://github.com/starship/starship/commit/7093d5cd84967edba93c9ed412b07519664f6356))
* **bash:** improve integration with bash-preexec ([#5734](https://github.com/starship/starship/issues/5734)) ([2aa711c](https://github.com/starship/starship/commit/2aa711ccc7096437e21149b18d1384534bfbcc57))
* **character:** also handle vi edit mode in pwsh ([#5775](https://github.com/starship/starship/issues/5775)) ([0891ec2](https://github.com/starship/starship/commit/0891ec27a40421cd742a853885731aed63f412aa))
* **direnv:** update to work with direnv v2.33 ([#5657](https://github.com/starship/starship/issues/5657)) ([cec111a](https://github.com/starship/starship/commit/cec111affdaf0a52f72c398f8307cf7e19c7dd8d))
* **git_branch:** fall back to "HEAD" when there is no current branch ([#5768](https://github.com/starship/starship/issues/5768)) ([6a96e84](https://github.com/starship/starship/commit/6a96e84a15e3ea598356e4fcad23ac4b2690dd1e))
* **nu:** continuation prompt not being displayed correctly ([#5851](https://github.com/starship/starship/issues/5851)) ([d308e91](https://github.com/starship/starship/commit/d308e918ee014f4ca9976683e79e819afd8160f7))
* **status:** fix pipestatus width calculation ([#5036](https://github.com/starship/starship/issues/5036)) ([ab84043](https://github.com/starship/starship/commit/ab840439e326a80c53466c7b767d29be0112b9d2)), closes [#3162](https://github.com/starship/starship/issues/3162)
* **zsh:** improve starship binary path escaping ([#5574](https://github.com/starship/starship/issues/5574)) ([2bb57cf](https://github.com/starship/starship/commit/2bb57cf0cd6d53194d26f4be96dff5fa14942622))

## [1.17.1](https://github.com/starship/starship/compare/v1.17.0...v1.17.1) (2024-01-02)


### Bug Fixes

* v1.17.0 post-release fix-ups ([#5660](https://github.com/starship/starship/issues/5660)) ([89dc192](https://github.com/starship/starship/commit/89dc19214bb671fe50a8f1be79a4594e7998ddea))


### Reverts

* refactor(modules): use whoami crate to get username ([#5669](https://github.com/starship/starship/issues/5669)) ([a83e107](https://github.com/starship/starship/commit/a83e10776ba37bd1ab439e5e4d0125a06e947728))

## [1.17.0](https://github.com/starship/starship/compare/v1.16.0...v1.17.0) (2023-12-28)


### Features

* add additional exit status code meanings from libc ([#5412](https://github.com/starship/starship/issues/5412)) ([81c7d0c](https://github.com/starship/starship/commit/81c7d0cc5805dc10018f0589a6671e1b727a0e9c))
* add typst module ([7b21705](https://github.com/starship/starship/commit/7b217056bdb8dcb5b328b51fa3b68fe837f9fb3c))
* **aws:** Adding the AWS SSO CLI env variable to profile list ([#5640](https://github.com/starship/starship/issues/5640)) ([6d96df3](https://github.com/starship/starship/commit/6d96df3c6828161bb9dc922fe45ef35a1ce33771))
* **direnv:** add new direnv module ([#5157](https://github.com/starship/starship/issues/5157)) ([e47bfba](https://github.com/starship/starship/commit/e47bfbabb9b7d6af12a29db9413a6ec03fba174b))
* **fossil_metrics:** add fossil_metrics module ([#4874](https://github.com/starship/starship/issues/4874)) ([e867cda](https://github.com/starship/starship/commit/e867cda1eb90ba452768bd2e0738afc2fd0db613))
* **hostname:** add detect_env_vars as option ([#5196](https://github.com/starship/starship/issues/5196)) ([43b2d42](https://github.com/starship/starship/commit/43b2d42cd526e34c5f0290e7409fbd6d3a54e908))
* **kubernetes:** Add styling based on current context ([#4550](https://github.com/starship/starship/issues/4550)) ([6b444e0](https://github.com/starship/starship/commit/6b444e05c688f9b871d0fe4624cd5559eba1f95c))
* R lang packages version, remove .Rprofile from rlang detection ([#5588](https://github.com/starship/starship/issues/5588)) ([5267c46](https://github.com/starship/starship/commit/5267c464eb5e4b23e44cdb7c56919991f4f67ae3))
* **scanner:** add option not to follow symlinks ([#5325](https://github.com/starship/starship/issues/5325)) ([7b851fc](https://github.com/starship/starship/commit/7b851fc30e109213e911eec38460315872f1ae59))
* **shell:** allow distinguishing between pwsh and powershell ([#5478](https://github.com/starship/starship/issues/5478)) ([d7a34b4](https://github.com/starship/starship/commit/d7a34b45f88ced63bd79a582c14a6b2f8ebd9544))


### Bug Fixes

* **bash:** unbound variable error with STARSHIP_PREEXEC_READY ([#5438](https://github.com/starship/starship/issues/5438)) ([8168c21](https://github.com/starship/starship/commit/8168c21293de8118af1e95778b1eee8f26cd6d6a))
* **docker_context:** ignore unix domain socket path from Docker Context ([#5616](https://github.com/starship/starship/issues/5616)) ([a910e09](https://github.com/starship/starship/commit/a910e094f77ba6d67349a561e5e9780becfe823a)), closes [#5548](https://github.com/starship/starship/issues/5548)
* **git_status:** Avoid printing error on missing stash ref ([#5434](https://github.com/starship/starship/issues/5434)) ([00d3dc8](https://github.com/starship/starship/commit/00d3dc86a21d11aede96f81ffbe49babe487984e))
* **git:** prevent `core.fsmonitor` from executing external commands ([#3981](https://github.com/starship/starship/issues/3981)) ([03278e4](https://github.com/starship/starship/commit/03278e4de4f540cbd0e346e9df878c7e6798d757))
* **install:** do not use curl installed through snap ([#5442](https://github.com/starship/starship/issues/5442)) ([0e73817](https://github.com/starship/starship/commit/0e738175c57d5789350b996b69c5713aac03835e))
* **pastel-powerline:** remove `$path` from docker-context module format string ([#5534](https://github.com/starship/starship/issues/5534)) ([6abc83d](https://github.com/starship/starship/commit/6abc83decdf176842985b4daa5b09771c6b93415))


### Performance Improvements

* **git_status:** avoid running in bare repos ([#5581](https://github.com/starship/starship/issues/5581)) ([ac4a839](https://github.com/starship/starship/commit/ac4a83910357d69950ca304a3fb41d1d39bc3592))
* Skip unnecessary indirection in starship init zsh ([#5322](https://github.com/starship/starship/issues/5322)) ([5ca8daa](https://github.com/starship/starship/commit/5ca8daacd4ce936f97170f814a780b34bfaa486e))

## [1.16.0](https://github.com/starship/starship/compare/v1.15.0...v1.16.0) (2023-07-30)


### Features

* **pwsh:** Support vi command mode indicator ([#5049](https://github.com/starship/starship/issues/5049)) ([3180509](https://github.com/starship/starship/commit/318050976b58f032a17ff4d122d21cb63faf8e3b))
* **shlvl:** add repeat_offset for repeated symbol ([#5289](https://github.com/starship/starship/issues/5289)) ([3402f0e](https://github.com/starship/starship/commit/3402f0e82aab907a77f0967abb99d4cde512c069))
* update the nushell init file and make it valid module and overlay ([#5188](https://github.com/starship/starship/issues/5188)) ([470aeb0](https://github.com/starship/starship/commit/470aeb09a72936a66937d68f96165a977b5c213d))


### Bug Fixes

* **bash:** Clear out completed jobs before counting NUM_JOBS ([#5253](https://github.com/starship/starship/issues/5253)) ([0a05b5c](https://github.com/starship/starship/commit/0a05b5ca8b7f147ead6d575297ef9f34191193af))
* **rprompt:** remove lprompt modules from `$all` again ([#5067](https://github.com/starship/starship/issues/5067)) ([b9a4b08](https://github.com/starship/starship/commit/b9a4b08ac24805061c5faaef98ccbee7b018f5f0))


### Performance Improvements

* **git_status:** query git stash count via gitoxide ([#5238](https://github.com/starship/starship/issues/5238)) ([410f107](https://github.com/starship/starship/commit/410f107c722a2f07737d4be01960f2621bf418dd))

## [1.15.0](https://github.com/starship/starship/compare/v1.14.2...v1.15.0) (2023-06-06)


### Features

* Add Solidity Module ([#5047](https://github.com/starship/starship/issues/5047)) ([b2ebd5b](https://github.com/starship/starship/commit/b2ebd5b50c62fe5eb1cf8f5b0f79deaff2edd059))
* add typechange to git_status module ([#4829](https://github.com/starship/starship/issues/4829)) ([edb96ca](https://github.com/starship/starship/commit/edb96cad580e5c414c34a4f64476a64a20595459))
* **aws:** support aws sso with automatic authentication refresh ([#5170](https://github.com/starship/starship/issues/5170)) ([297176b](https://github.com/starship/starship/commit/297176b0b8b9da34176d7b278837f77f960799b1))
* **azure:** subscription name aliases ([#4949](https://github.com/starship/starship/issues/4949)) ([27ffa37](https://github.com/starship/starship/commit/27ffa37cfdf2eff9874e543f88fa389bf5c2dae3))
* **gcloud:** add `detect_env_vars` option ([#5166](https://github.com/starship/starship/issues/5166)) ([d07a8e3](https://github.com/starship/starship/commit/d07a8e3668838223aeeb94e810a0b29806e35f78))
* **git_metrics:** add option to ignore submodules ([#5052](https://github.com/starship/starship/issues/5052)) ([ce01423](https://github.com/starship/starship/commit/ce014231521c981260ff7c1018acf694c65c97fe))
* **golang:** adding `mod_version` variable ([#5177](https://github.com/starship/starship/issues/5177)) ([351bf9d](https://github.com/starship/starship/commit/351bf9d0b382adcc3e073c1a293fd815bb623f37))
* **nodejs:** Add `expected_version` variable ([#5081](https://github.com/starship/starship/issues/5081)) ([70d2014](https://github.com/starship/starship/commit/70d2014f3447e616fb45b63b2793b256e19aa631))


### Bug Fixes

* bump libz-ng-sys ([#5218](https://github.com/starship/starship/issues/5218)) ([6ab8f40](https://github.com/starship/starship/commit/6ab8f4061fe816a78a310c01a993a5a4690369ff))
* **config:** Make print-config not panic without a config ([#5001](https://github.com/starship/starship/issues/5001)) ([ce7f984](https://github.com/starship/starship/commit/ce7f984932a97b4ad3cd6e6ece8e1c3b6022ba99))
* ensure nested style variables are processed during formatting ([e5cec9e](https://github.com/starship/starship/commit/e5cec9ea50963a45bb1c209abc747ee1983dcabd))
* **presets:** Added ($style) to format in module 'sudo' in Bracketed Segments Preset ([#5146](https://github.com/starship/starship/issues/5146)) ([1bd6db5](https://github.com/starship/starship/commit/1bd6db58307c1945c3b0cabec8d6663730394377))
* **snap:** Update snapcraft.yaml to add personal-files interface ([#5131](https://github.com/starship/starship/issues/5131)) ([b3ccc0f](https://github.com/starship/starship/commit/b3ccc0f05e451ada800d233613ef32756682249f))
* **style:** ensure nested style variables are processed during formatting ([#5120](https://github.com/starship/starship/issues/5120)) ([e5cec9e](https://github.com/starship/starship/commit/e5cec9ea50963a45bb1c209abc747ee1983dcabd))
* update of presets and default configuration to reflect changes in Nerd Fonts 3.0 ([#5162](https://github.com/starship/starship/issues/5162)) ([2558c45](https://github.com/starship/starship/commit/2558c4588b5bcc404df474c948de0b72b109be01))

## [1.14.2](https://github.com/starship/starship/compare/v1.14.1...v1.14.2) (2023-04-12)


### Bug Fixes

* **git_commit:** resolve panic on 32-bit targets ([#5095](https://github.com/starship/starship/issues/5095)) ([5ef90a6](https://github.com/starship/starship/commit/5ef90a615f73a9f240a3c63ab601db1302adb01d))

## [1.14.1](https://github.com/starship/starship/compare/v1.14.0...v1.14.1) (2023-04-11)


### Bug Fixes

* bootstrap manifest for release-please ([#5087](https://github.com/starship/starship/issues/5087)) ([e392d14](https://github.com/starship/starship/commit/e392d14f4eb65d8761ea8bafb498d2a0d966dcef))
* trigger release ([8bdb953](https://github.com/starship/starship/commit/8bdb953ad85068e182878c3295a94559a608ee31))
* update the release-please manifest ([cd501ec](https://github.com/starship/starship/commit/cd501ecd9fb4b898d6d4472b46471a05a42b6052))

## [1.14.0](https://github.com/starship/starship/compare/v1.13.1...v1.14.0) (2023-04-10)


### Features

* **aws:** add support for source_profile ([#3834](https://github.com/starship/starship/issues/3834)) ([d2801ac](https://github.com/starship/starship/commit/d2801ac44301dcef1f87ab5fd26abee36997f71d))
* **aws:** add support for source_profile ([#4859](https://github.com/starship/starship/issues/4859)) ([d2801ac](https://github.com/starship/starship/commit/d2801ac44301dcef1f87ab5fd26abee36997f71d))
* **aws:** Adds support for AWS_CREDENTIAL_EXPIRATION environment variable ([#5002](https://github.com/starship/starship/issues/5002)) ([74ce7fd](https://github.com/starship/starship/commit/74ce7fdbee071c28c77fd148d4ba02515f272d10))
* **custom:** add option to check if pwd is in a repo ([#4822](https://github.com/starship/starship/issues/4822)) ([d29ce7c](https://github.com/starship/starship/commit/d29ce7c45d4ea21a6e14ad308bd50cb0e61d1ef8))
* **fossil:** detection of Fossil check-outs in subdirectories ([#4910](https://github.com/starship/starship/issues/4910)) ([4bca74e](https://github.com/starship/starship/commit/4bca74eca29e159f0d6f27db432927012848408c))
* **release:** handle chocolatey starship.portable and starship.install pkg publishing ([#4723](https://github.com/starship/starship/issues/4723)) ([b55774d](https://github.com/starship/starship/commit/b55774d3a68b32c0ed17983adeb6355e75c65f6b))


### Bug Fixes

* **fossil_branch:** fossil checkout database file name on windows ([#4978](https://github.com/starship/starship/issues/4978)) ([c07a21d](https://github.com/starship/starship/commit/c07a21d48abe4e01a96a2d1b641876207e8d02fb))
* **fossil_branch:** use proper fossil checkout database file name on windows ([c07a21d](https://github.com/starship/starship/commit/c07a21d48abe4e01a96a2d1b641876207e8d02fb))
* **gradle:** add support for unstable Gradle versions ([#5021](https://github.com/starship/starship/issues/5021)) ([f7fe41f](https://github.com/starship/starship/commit/f7fe41f9c6c455e8ced284ad2d55d2a51a5da748))
* **init:** avoid cygpath for starship binary path ([#4970](https://github.com/starship/starship/issues/4970)) ([0ad0465](https://github.com/starship/starship/commit/0ad0465a7a3296b3223693c655f370b7aae0d441))
* **java:** wrong version number when using Android Studio JDK ([#4966](https://github.com/starship/starship/issues/4966)) ([de7e948](https://github.com/starship/starship/commit/de7e94884bc309814f6af79d68d664efb513e093))
* **preset:** add output-flag to avoid encoding issues ([#4926](https://github.com/starship/starship/issues/4926)) ([5e78226](https://github.com/starship/starship/commit/5e78226a3fbe722331f6f0a1d352bbc48d38247f))
* **pulumi:** Fix formatting on pulumi module when using version ([#5038](https://github.com/starship/starship/issues/5038)) ([aef799b](https://github.com/starship/starship/commit/aef799bfb089c5d259354208a6bcd5a0b639888f))

## [1.13.1](https://github.com/starship/starship/compare/v1.13.0...v1.13.1) (2023-02-26)


### Bug Fixes

* trigger release ([ff82fb9](https://github.com/starship/starship/commit/ff82fb99af88c007a18c7655fb0150c4415bb5db))

## [1.13.0](https://github.com/starship/starship/compare/v1.12.0...v1.13.0) (2023-02-24)


### Features

* add pijul_channel module ([#4765](https://github.com/starship/starship/issues/4765)) ([67b6376](https://github.com/starship/starship/commit/67b6376e2ef0835350e3e856ade6602b6c187c42))
* **config:** Adds support for --profile &lt;custom profile name&gt; ([#3467](https://github.com/starship/starship/issues/3467)) ([10433e3](https://github.com/starship/starship/commit/10433e31effb4040c47d02d565d1643bcf984fa6))
* **env_var:** Add support for env_var.VAR in format ([#4497](https://github.com/starship/starship/issues/4497)) ([5d4cb6f](https://github.com/starship/starship/commit/5d4cb6ff8f6bd1915aa2c16162950b270f1759b1))
* **fennel:** add fennel module ([#4717](https://github.com/starship/starship/issues/4717)) ([e93dbf8](https://github.com/starship/starship/commit/e93dbf86301e19a89bd64997d95ba63a64f473aa))
* **fossil_branch:** add fossil_branch module ([#4806](https://github.com/starship/starship/issues/4806)) ([41eb98b](https://github.com/starship/starship/commit/41eb98b310cd8134cec7bd8dcb55869a984653cf))
* **gradle:** add gradle module ([#4423](https://github.com/starship/starship/issues/4423)) ([220844d](https://github.com/starship/starship/commit/220844daa014046bedbc9ce703f8b18fbe267e3c))
* **hg_branch:** Add support for mercurial topics and find hg root dir ([#4771](https://github.com/starship/starship/issues/4771)) ([8d2256a](https://github.com/starship/starship/commit/8d2256ab1d0ba288fb6ba9b9248bc2210ca01059))
* **java:** Add `.sdkmanrc` for Java ([#4888](https://github.com/starship/starship/issues/4888)) ([07c2298](https://github.com/starship/starship/commit/07c2298965ee67300319c012bdf5fadbc8db4931))
* **logger:** delete old logs & avoid more dup logs ([#4348](https://github.com/starship/starship/issues/4348)) ([e47ea57](https://github.com/starship/starship/commit/e47ea57db21125372aeeae87ce555855a98adaab))
* **nix:** support new `nix shell` command ([#4724](https://github.com/starship/starship/issues/4724)) ([19fdf9b](https://github.com/starship/starship/commit/19fdf9bba59f6ae5a756b81d221a9dc3185208f5))


### Bug Fixes

* 'to to' -&gt; 'to' ([8c2135f](https://github.com/starship/starship/commit/8c2135f55d4a8b1026ce3cf7055efde6ab47d13d))
* **container:** reduce docker, podman and systemd confusion ([#4832](https://github.com/starship/starship/issues/4832)) ([85d683d](https://github.com/starship/starship/commit/85d683daf235854ffc356354c6b3ba7096de6193))
* **fish:** enable transient prompt when in vi mode ([#4826](https://github.com/starship/starship/issues/4826)) ([9ac924e](https://github.com/starship/starship/commit/9ac924eb3f0f8faa6da0375d92fc1dc22b8ba721))
* **git_commit:** fix potential test failure ([#4734](https://github.com/starship/starship/issues/4734)) ([27d167b](https://github.com/starship/starship/commit/27d167b7a202cd1da39a731813df155dacb4c81b))
* Improve regex for extracting gradle package version from gradle.properties ([#4759](https://github.com/starship/starship/issues/4759)) ([9093891](https://github.com/starship/starship/commit/9093891acbe2c86b1615c37386dadbb0cc632199))
* let-env warning when using nushell ([#4893](https://github.com/starship/starship/issues/4893)) ([e6c5571](https://github.com/starship/starship/commit/e6c5571fc9c1f47c711d5fcdd1799ced5b546454))
* **nodejs:** apply `style` even if node version is unavailable ([#4713](https://github.com/starship/starship/issues/4713)) ([e88484d](https://github.com/starship/starship/commit/e88484d5674b7c038346ff1c89089e535d2e2d6d))
* **package:** Improve regex for extracting gradle version from gradle.properties ([#4760](https://github.com/starship/starship/issues/4760)) ([9093891](https://github.com/starship/starship/commit/9093891acbe2c86b1615c37386dadbb0cc632199))
* Remove vulnerable time-0.1.x chrono dependency ([#4750](https://github.com/starship/starship/issues/4750)) ([255f91c](https://github.com/starship/starship/commit/255f91c3ce896f71b874f260b61f86232485d823))

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
