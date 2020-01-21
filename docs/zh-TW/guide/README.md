<p align="center">
  <br /><img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt" />
</p>
<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://github.com/starship/starship/workflows/Main%20workflow/badge.svg?branch=master&event=push"
      alt="GitHub Actions workflow status" /></a>
  <a href="https://crates.io/crates/starship"
    ><img src="https://img.shields.io/crates/v/starship" alt="Crates.io version" /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://repology.org/badge/tiny-repos/starship.svg"
      alt="Packaging status" /></a
><br />
    <a href="https://discord.gg/8Jzqu3T"
    ><img
      src="https://img.shields.io/discord/567163873606500352?logo=discord"
      alt="在 Discord 上聊天" /></a>
  <!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
<a href="#contributors"><img src="https://img.shields.io/badge/all%20contributors-68-orange" alt="All Contributors"></a>
<!-- ALL-CONTRIBUTORS-BADGE:END -->
</p>

<h4 align="center">
  <br />
  <a href="https://starship.rs/zh-TW/">網站</a>
  ·
  <a href="#-安裝">安裝</a>
  ·
  <a href="https://starship.rs/zh-TW/config/">設定</a>
</h4>
<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png" alt="English" /></a>
  &nbsp;
  <a href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png" alt="日本語" /></a>
  &nbsp;
  <a href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png" alt="繁體中文" /></a>
  &nbsp;
  <a href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png" alt="Русский" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/zh-CN"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png" alt="简体中文" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/de"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png" alt="Deutsch" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/es"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png" alt="Español" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/fr"
    ><img height="20" src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png" alt="Français" /></a>
</p>

<h1></h1>

<p align="center"> Starship 是一個可以用於任何 shell、極小、極快、可高度客製化的提示字元！<br /> 這個提示字元顯示你工作時需要的資訊，同時又順暢又不會打擾你。 <p>

<p align="center">
  <br>
  <img alt="Starship with iTerm2 and the Snazzy theme" src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif" width="80%">
  <br>
  <br>
</p>

## 🍬 特色

- 如果最近一個指令結束時回傳非零的代碼，提示字元將轉成紅色
- 現在的 Go 版本 (`🐹`)
- Current Java version (`☕`)
- Current Node.js version (`⬢`)
- Current PHP version (`🐘`)
- 現在的 Python 版本 (`🐍`)
- 現在的 Ruby 版本 (`💎`)
- 現在的 Rust 版本 (`🦀`)
- Current .NET version (`•NET`)
- 現在資料夾中的組件 (package) 版本 (`📦`)
  - npm (Node.js)
  - cargo (Rust)
  - poetry (Python)
  - composer (PHP)
- 現在的 Git branch 與豐富的程式庫 (repository) 狀態：
  - `=` — 修改衝突
  - `⇡` — 超前遠端 (remote) branch
  - `⇣` — 落後遠端 (remote) branch
  - `⇕` — 修改發散 (diverge)
  - `?` — 修改未追蹤
  - `$` — 隱藏的 (stashed) 修改
  - `!` — 修改過的檔案
  - `+` — 新增的檔案
  - `»` — 重新命名的檔案
  - `✘` — 刪除的檔案
- Current Mercurial branch
- 現在的電池電量與狀態
  - `⇡` – charging
  - `⇣` – discharging
  - `•` – fully charged
- 標註背景執行的工作 (`✦`)
- 現在的 Kubernetes 叢集以及名稱空間 (Namespace) (`☸`)
- Current Amazon Web Services (AWS) profile (`☁️`)
- Execution time of the last command
- Custom environment variable value
- 偵測 Nix-shell 的環境
- 會在使用者與登入的使用者不同時顯示使用者名稱
- Optional current time in 12/24hr format
- Current Terraform Workspace and version (`💠`)
- Current Conda environment (`C`)

## 🚀 安裝

### 先決要求

- 一個已經安裝並在你的終端機內啟動的 [Powerline 字型](https://github.com/powerline/fonts) (例如，試試看 [Fira Code](https://github.com/tonsky/FiraCode))。

### 入門

1. 安裝 **starship** 執行檔：


   #### Install Latest Version


   ##### From prebuilt binary, with Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### From source on [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```


   #### Install via Package Manager


   ##### With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. 將初始化腳本 (script) 加入你的 shell 的設定檔：


   #### Bash

   將以下內容放到 `~/.bashrc` 的結尾：

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   將以下內容放到 `~/.config/fish/config.fish` 的結尾：

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   將以下內容放到 `~/.zshrc` 的結尾：

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   將以下內容放到 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 的結尾 (或是在 -Nix 上的 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`)：

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🔧 設定

關於如何設定 Starship，請看 [文件](https://starship.rs/zh-TW/config/) 。

## 🤝 貢獻

我們歡迎具有**各式各樣能力**的貢獻者！ 如果你正在尋找容易加入的方法，試試看標註為「[good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)」的 issue。

### 高優先需求

- 👩‍💼 **專案管理員**
  - 我們有一個 GitHub 專案以及許多尚未整理/排序的功能，甚至還有點子尚未被製作成 issue。 Starship 需要有人掌管產品的方向！
- 👩‍🎨 **設計師**
  - 喜歡製作吸睛的網站？ 超讚！ 我們正在想辦法建立一個漂亮的 landing page 來展示 Starship 的美妙。 幫助設計 Starship 的商標正好是一個試驗新點子的機會！
- 👩‍💻 **Rust 開發者**
  - 當正在寫慣用的 Rust 程式碼、設計高效的 Rust 程式架構、效能優化、跨平台建置優化、以及其他很多東西時，可以在這裡看到_許多_唾手可得的東西！ 我 ([@matchai](https://github.com/matchai)) 是一個 Rust 新手。 來為我們指出正確的方向吧！

如果你對貢獻 Starship 有興趣，請看我們的 [貢獻指南](https://github.com/starship/starship/blob/master/CONTRIBUTING.md) 。 另外，請不用客氣加入我們的 [Discord 伺服器](https://discord.gg/8Jzqu3T) 並來問候一下。 👋

### 貢獻者們

感謝這些優秀的人們 ([表情符號索引](https://allcontributors.org/docs/en/emoji-key))：

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://twitter.com/matchai"><img src="https://avatars0.githubusercontent.com/u/4658208?v=4" width="100px;" alt="" /><br /><sub><b>Matan Kushner</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=matchai" title="Code">💻</a> <a href="#design-matchai" title="Design">🎨</a> <a href="#ideas-matchai" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-matchai" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-matchai" title="Maintenance">🚧</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Amatchai" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=matchai" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/johnletey"><img src="https://avatars0.githubusercontent.com/u/30328854?v=4" width="100px;" alt="" /><br /><sub><b>John Letey</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=johnletey" title="Code">💻</a> <a href="#ideas-johnletey" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Ajohnletey" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=johnletey" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://timmulqueen.com"><img src="https://avatars1.githubusercontent.com/u/6132021?v=4" width="100px;" alt="" /><br /><sub><b>Tim Mulqueen</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Multimo" title="Code">💻</a> <a href="#ideas-Multimo" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3AMultimo" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=Multimo" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/sirMerr"><img src="https://avatars2.githubusercontent.com/u/11183523?v=4" width="100px;" alt="" /><br /><sub><b>Tiffany Le-Nguyen</b></sub></a><br /><a href="#ideas-sirMerr" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-sirMerr" title="Maintenance">🚧</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3AsirMerr" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=sirMerr" title="Documentation">📖</a></td>
    <td align="center"><a href="https://about.snuggi.es"><img src="https://avatars0.githubusercontent.com/u/26250962?v=4" width="100px;" alt="" /><br /><sub><b>​Snuggle</b></sub></a><br /><a href="#design-Snuggle" title="Design">🎨</a> <a href="#ideas-Snuggle" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-Snuggle" title="Maintenance">🚧</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3ASnuggle" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/mehcode"><img src="https://avatars1.githubusercontent.com/u/753919?v=4" width="100px;" alt="" /><br /><sub><b>Ryan Leckey</b></sub></a><br /><a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Amehcode" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/youssefhabri"><img src="https://avatars3.githubusercontent.com/u/1578005?v=4" width="100px;" alt="" /><br /><sub><b>Youssef Habri</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=youssefhabri" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/chipbuster"><img src="https://avatars2.githubusercontent.com/u/4605384?v=4" width="100px;" alt="" /><br /><sub><b>Kevin Song</b></sub></a><br /><a href="https://github.com/starship/starship/issues?q=author%3Achipbuster" title="Bug reports">🐛</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://andrewda.me"><img src="https://avatars1.githubusercontent.com/u/10191084?v=4" width="100px;" alt="" /><br /><sub><b>Andrew Dassonville</b></sub></a><br /><a href="https://github.com/starship/starship/issues?q=author%3Aandrewda" title="Bug reports">🐛</a> <a href="https://github.com/starship/starship/commits?author=andrewda" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/MaT1g3R"><img src="https://avatars1.githubusercontent.com/u/15258494?v=4" width="100px;" alt="" /><br /><sub><b>MaT1g3R</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=MaT1g3R" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=MaT1g3R" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=MaT1g3R" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/AZanellato"><img src="https://avatars3.githubusercontent.com/u/30451287?v=4" width="100px;" alt="" /><br /><sub><b>André Zanellato</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=AZanellato" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=AZanellato" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=AZanellato" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://saghm.com"><img src="https://avatars2.githubusercontent.com/u/5875560?v=4" width="100px;" alt="" /><br /><sub><b>Saghm Rossi</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=saghm" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=saghm" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=saghm" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://medium.com/@cappyzawa"><img src="https://avatars3.githubusercontent.com/u/12455284?v=4" width="100px;" alt="" /><br /><sub><b>Shu Kutsuzawa</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=cappyzawa" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=cappyzawa" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=cappyzawa" title="Tests">⚠️</a> <a href="#translation-cappyzawa" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/iamsauravsharma"><img src="https://avatars0.githubusercontent.com/u/38726015?v=4" width="100px;" alt="" /><br /><sub><b>Saurav Sharma</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=iamsauravsharma" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=iamsauravsharma" title="Documentation">📖</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/andytom"><img src="https://avatars1.githubusercontent.com/u/108836?v=4" width="100px;" alt="" /><br /><sub><b>Thomas O'Donnell</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=andytom" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=andytom" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/commits?author=andytom" title="Documentation">📖</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Aandytom" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/bbigras"><img src="https://avatars1.githubusercontent.com/u/24027?v=4" width="100px;" alt="" /><br /><sub><b>Bruno Bigras</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=bbigras" title="Code">💻</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Abbigras" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://neilkistner.com/"><img src="https://avatars1.githubusercontent.com/u/186971?v=4" width="100px;" alt="" /><br /><sub><b>Neil Kistner</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=wyze" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=wyze" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Awyze" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="http://ca.linkedin.com/in/qstrahl"><img src="https://avatars3.githubusercontent.com/u/2235277?v=4" width="100px;" alt="" /><br /><sub><b>Quinn Strahl</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=qstrahl" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=qstrahl" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/tivervac"><img src="https://avatars2.githubusercontent.com/u/3389524?v=4" width="100px;" alt="" /><br /><sub><b>Titouan Vervack</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=tivervac" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=tivervac" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://nosubstance.me"><img src="https://avatars1.githubusercontent.com/u/1269815?v=4" width="100px;" alt="" /><br /><sub><b>Francisco Lopes</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=oblitum" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ahouts"><img src="https://avatars1.githubusercontent.com/u/16907671?v=4" width="100px;" alt="" /><br /><sub><b>Andrew Houts</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=ahouts" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=ahouts" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=ahouts" title="Tests">⚠️</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/nickwb"><img src="https://avatars2.githubusercontent.com/u/594211?v=4" width="100px;" alt="" /><br /><sub><b>Nick Young</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=nickwb" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=nickwb" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=nickwb" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Anickwb" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/g2p"><img src="https://avatars1.githubusercontent.com/u/61678?v=4" width="100px;" alt="" /><br /><sub><b>Gabriel de Perthuis</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=g2p" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Hofer-Julian"><img src="https://avatars1.githubusercontent.com/u/30049909?v=4" width="100px;" alt="" /><br /><sub><b>Hofer-Julian</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Hofer-Julian" title="Documentation">📖</a></td>
    <td align="center"><a href="http://blog.unhappychoice.com"><img src="https://avatars3.githubusercontent.com/u/5608948?v=4" width="100px;" alt="" /><br /><sub><b>Yuji Ueki</b></sub></a><br /><a href="#content-unhappychoice" title="Content">🖋</a> <a href="#translation-unhappychoice" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/heyrict"><img src="https://avatars3.githubusercontent.com/u/25698503?v=4" width="100px;" alt="" /><br /><sub><b>谢祯晖</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=heyrict" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=heyrict" title="Documentation">📖</a> <a href="#translation-heyrict" title="Translation">🌍</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Aheyrict" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://twitter.com/bookun2851"><img src="https://avatars2.githubusercontent.com/u/10346162?v=4" width="100px;" alt="" /><br /><sub><b>Kutsuzawa Ryo</b></sub></a><br /><a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Abookun" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=bookun" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=bookun" title="Tests">⚠️</a> <a href="#translation-bookun" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/hdevalke"><img src="https://avatars1.githubusercontent.com/u/2261239?v=4" width="100px;" alt="" /><br /><sub><b>hdevalke</b></sub></a><br /><a href="#ideas-hdevalke" title="Ideas, Planning, & Feedback">🤔</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/jakubclark"><img src="https://avatars0.githubusercontent.com/u/19486495?v=4" width="100px;" alt="" /><br /><sub><b>Kuba Clark</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=jakubclark" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=jakubclark" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=jakubclark" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://breax.org"><img src="https://avatars2.githubusercontent.com/u/862483?v=4" width="100px;" alt="" /><br /><sub><b>Gimbar</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=gimbar" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=gimbar" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/commits?author=gimbar" title="Documentation">📖</a></td>
    <td align="center"><a href="http://tomhotston.net"><img src="https://avatars0.githubusercontent.com/u/22729355?v=4" width="100px;" alt="" /><br /><sub><b>Tom Hotston</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=TomHotston" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=TomHotston" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/bijancn"><img src="https://avatars3.githubusercontent.com/u/2117164?v=4" width="100px;" alt="" /><br /><sub><b>Bijan Chokoufe Nejad</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=bijancn" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=bijancn" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/pulls?q=is%3Apr+reviewed-by%3Abijancn" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/yuri1969"><img src="https://avatars3.githubusercontent.com/u/13468636?v=4" width="100px;" alt="" /><br /><sub><b>yuri</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=yuri1969" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=yuri1969" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=yuri1969" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/TsubasaKawajiri"><img src="https://avatars2.githubusercontent.com/u/39114857?v=4" width="100px;" alt="" /><br /><sub><b>TsubasaKawajiri</b></sub></a><br /><a href="#translation-TsubasaKawajiri" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/qryxip"><img src="https://avatars2.githubusercontent.com/u/14125495?v=4" width="100px;" alt="" /><br /><sub><b>Ryo Yamashita</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=qryxip" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://pbzweihander.github.io"><img src="https://avatars2.githubusercontent.com/u/15262528?v=4" width="100px;" alt="" /><br /><sub><b>Thomas Lee</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=pbzweihander" title="Code">💻</a></td>
    <td align="center"><a href="https://pt2121.github.io"><img src="https://avatars0.githubusercontent.com/u/616399?v=4" width="100px;" alt="" /><br /><sub><b>(´⌣`ʃƪ)</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=pt2121" title="Code">💻</a></td>
    <td align="center"><a href="https://southcla.ws"><img src="https://avatars1.githubusercontent.com/u/1636971?v=4" width="100px;" alt="" /><br /><sub><b>Barnaby Keene</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Southclaws" title="Code">💻</a></td>
    <td align="center"><a href="http://keawade.io/"><img src="https://avatars2.githubusercontent.com/u/7308850?v=4" width="100px;" alt="" /><br /><sub><b>Keith Wade</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=keawade" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=keawade" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/LukeAI"><img src="https://avatars3.githubusercontent.com/u/43993778?v=4" width="100px;" alt="" /><br /><sub><b>LukeAI</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=LukeAI" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/zekesonxx"><img src="https://avatars1.githubusercontent.com/u/965509?v=4" width="100px;" alt="" /><br /><sub><b>Zach Mertes</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=zekesonxx" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=zekesonxx" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=zekesonxx" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/davidkna"><img src="https://avatars2.githubusercontent.com/u/835177?v=4" width="100px;" alt="" /><br /><sub><b>David Knaack</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=davidkna" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=davidkna" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=davidkna" title="Tests">⚠️</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/CSumm"><img src="https://avatars1.githubusercontent.com/u/31711543?v=4" width="100px;" alt="" /><br /><sub><b>Carl Summers</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=CSumm" title="Documentation">📖</a></td>
    <td align="center"><a href="http://www.slmt.tw"><img src="https://avatars2.githubusercontent.com/u/6824412?v=4" width="100px;" alt="" /><br /><sub><b>Yushan Lin</b></sub></a><br /><a href="#translation-SLMT" title="Translation">🌍</a></td>
    <td align="center"><a href="https://weihanglo.tw"><img src="https://avatars2.githubusercontent.com/u/14314532?v=4" width="100px;" alt="" /><br /><sub><b>Weihang Lo</b></sub></a><br /><a href="#translation-weihanglo" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/pinshan"><img src="https://avatars0.githubusercontent.com/u/7709675?v=4" width="100px;" alt="" /><br /><sub><b>pinshan</b></sub></a><br /><a href="#translation-pinshan" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/brianlow"><img src="https://avatars2.githubusercontent.com/u/938138?v=4" width="100px;" alt="" /><br /><sub><b>Brian Low</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=brianlow" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=brianlow" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=brianlow" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://matiaskotlik.github.io"><img src="https://avatars2.githubusercontent.com/u/20362627?v=4" width="100px;" alt="" /><br /><sub><b>Matias Kotlik</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=matiaskotlik" title="Code">💻</a></td>
    <td align="center"><a href="https://marblenix.com"><img src="https://avatars0.githubusercontent.com/u/6401427?v=4" width="100px;" alt="" /><br /><sub><b>marblenix</b></sub></a><br /><a href="#infra-marblenix" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="https://github.com/starship/starship/commits?author=marblenix" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=marblenix" title="Tests">⚠️</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/weirongxu"><img src="https://avatars3.githubusercontent.com/u/1709861?v=4" width="100px;" alt="" /><br /><sub><b>Raidou</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=weirongxu" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=weirongxu" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/rpl"><img src="https://avatars1.githubusercontent.com/u/11484?v=4" width="100px;" alt="" /><br /><sub><b>Luca Greco</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=rpl" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=rpl" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=rpl" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://lucar.in"><img src="https://avatars2.githubusercontent.com/u/6934358?v=4" width="100px;" alt="" /><br /><sub><b>Luca Rinaldi</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=lucarin91" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/REBELinBLUE"><img src="https://avatars1.githubusercontent.com/u/2143908?v=4" width="100px;" alt="" /><br /><sub><b>Stephen Ball</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=REBELinBLUE" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=REBELinBLUE" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/commits?author=REBELinBLUE" title="Documentation">📖</a></td>
    <td align="center"><a href="http://about.houqp.me"><img src="https://avatars0.githubusercontent.com/u/670302?v=4" width="100px;" alt="" /><br /><sub><b>Qingping Hou</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=houqp" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=houqp" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=houqp" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/wendorf"><img src="https://avatars3.githubusercontent.com/u/407342?v=4" width="100px;" alt="" /><br /><sub><b>Dan Wendorf</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=wendorf" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=wendorf" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://popey.com/"><img src="https://avatars0.githubusercontent.com/u/1841272?v=4" width="100px;" alt="" /><br /><sub><b>Alan Pope</b></sub></a><br /><a href="#infra-popey" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/BuggStream"><img src="https://avatars1.githubusercontent.com/u/51194915?v=4" width="100px;" alt="" /><br /><sub><b>BuggStream</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=BuggStream" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=BuggStream" title="Documentation">📖</a></td>
    <td align="center"><a href="http://jonstodle.com"><img src="https://avatars1.githubusercontent.com/u/1719761?v=4" width="100px;" alt="" /><br /><sub><b>Jon Grythe Stødle</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=jonstodle" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=jonstodle" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://jasonet.co"><img src="https://avatars1.githubusercontent.com/u/10660468?v=4" width="100px;" alt="" /><br /><sub><b>Jason Etcovitch</b></sub></a><br /><a href="https://github.com/starship/starship/issues?q=author%3AJasonEtco" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/etiennemabille"><img src="https://avatars3.githubusercontent.com/u/11175343?v=4" width="100px;" alt="" /><br /><sub><b>Etienne Mabille</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=etiennemabille" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Scotsguy"><img src="https://avatars3.githubusercontent.com/u/20385973?v=4" width="100px;" alt="" /><br /><sub><b>AppleTheGolden</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Scotsguy" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=Scotsguy" title="Documentation">📖</a></td>
    <td align="center"><a href="http://sda.io"><img src="https://avatars1.githubusercontent.com/u/481987?v=4" width="100px;" alt="" /><br /><sub><b>Mike Sampson</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=mfs" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=mfs" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=mfs" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://sternentstehung.de"><img src="https://avatars3.githubusercontent.com/u/36575275?v=4" width="100px;" alt="" /><br /><sub><b>Dominik Braun</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=dominikbraun" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/dten"><img src="https://avatars0.githubusercontent.com/u/1019038?v=4" width="100px;" alt="" /><br /><sub><b>David Hewson</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=dten" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/gkeep"><img src="https://avatars3.githubusercontent.com/u/20600053?v=4" width="100px;" alt="" /><br /><sub><b>Gennady Koshkin</b></sub></a><br /><a href="#translation-gkeep" title="Translation">🌍</a></td>
    <td align="center"><a href="https://blog.brightone.space"><img src="https://avatars1.githubusercontent.com/u/12615679?v=4" width="100px;" alt="" /><br /><sub><b>Oleksii Filonenko</b></sub></a><br /><a href="#translation-filalex77" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/ivanovart"><img src="https://avatars2.githubusercontent.com/u/5867379?v=4" width="100px;" alt="" /><br /><sub><b>Artem Ivanov</b></sub></a><br /><a href="#translation-ivanovart" title="Translation">🌍</a></td>
    <td align="center"><a href="http://www.drivendata.org"><img src="https://avatars3.githubusercontent.com/u/1799186?v=4" width="100px;" alt="" /><br /><sub><b>Peter Bull</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=pjbull" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=pjbull" title="Tests">⚠️</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

這個專案遵守 [all-contributors](https://github.com/all-contributors/all-contributors) 規範。 歡迎任何種類的貢獻者！

## 💭 發想來自

請看之前這些幫助我們創造 Starship 的前任作品。 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - 給太空人的 ZSH 提示。

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - 使用 Javascript 撰寫的跨 shell robbyrussell 主題。

- **[reujab/silver](https://github.com/reujab/silver)** - 一個跨 shell、可客製化、像 powerline 的圖案提示字元。

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 許可

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> 這個專案使用 [ISC](https://github.com/starship/starship/blob/master/LICENSE) 許可。
