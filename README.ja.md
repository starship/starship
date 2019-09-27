<p align="center">
  <br />
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt"
  />
</p>

<p align="center">
  <a href="https://crates.io/crates/starship">
    <img src="https://badgen.net/crates/v/starship" alt="Crates.io version" />
  </a>
  <a href="https://dev.azure.com/starship-control/starship/_build">
    <img
      src="https://badgen.net/azure-pipelines/starship-control/starship/Starship%20Test%20Suite"
      alt="Azure Pipelines Build Status"
    />
  </a>
    <a href="https://repology.org/project/starship/versions">
    <img src="https://repology.org/badge/tiny-repos/starship.svg" alt="Packaging status">
  </a><br >
  <a href="#-貢献">
    <img
      src="https://badgen.net/badge/all%20contributors/20/orange"
      alt="All Contributors"
    />
  </a>
  <a href="https://discord.gg/8Jzqu3T">
    <img
      src="https://badgen.net/badge/chat/on%20discord/7289da"
      alt="Chat on Discord"
    />
  </a>
</p>

<h4 align="center">
  <br />
  <a href="https://starship.rs">ウェブサイト</a>
  ·
  <a href="#-installation">インストール</a>
  ·
  <a href="https://starship.rs/config/">設定</a>
</h4>

<h1></h1>

<

p align="center"> Starship は小さく、とても高速に動作し、非常にカスタマイズ可能なすべてのシェルに対応したプロンプトです！  
プロンプトには、作業中に必要な情報が表示されます。

<

p>

<p align="center">
  <br >
  <img alt="Starship with iTerm2 and the Snazzy theme" src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif" width="80%">
  <br >
  <br >
</p>

## 🍬 機能

- 最後のコマンドがゼロ以外のコードで終了すると、プロンプト文字が赤に変わります
- ログインユーザーと異なる場合にユーザー名を表示する
- Current Java version(`☕`)
- Current Node.js version(`⬢`)
- Current Rust version (`🦀`)
- Current Ruby version (`💎`)
- Current Python version (`🐍`)
- Current Go version (`🐹`)
- Nix-shell environment detection
- Print an environment variable
- Current version of package in current directory (`📦`) 
  - npm (Node.js)
  - cargo (Rust)
  - poetry (Python)
- Current battery level and status
- Current Git branch and rich repo status: 
  - `=` — conflicting changes
  - `⇡` — ahead of remote branch
  - `⇣` — behind of remote branch
  - `⇕` — diverged changes
  - `?` — untracked changes
  - `$` — stashed changes
  - `!` — modified files
  - `+` — added files
  - `»` — renamed files
  - `✘` — deleted files
- Execution time of the last command if it exceeds the set threshold
- Indicator for jobs in the background (`✦`)

## 🚀 インストール

### 必要なもの

- [Powerline フォント](https://github.com/powerline/fonts) がターミナルにインストールされて有効になっている必要があります（例えば [Fira Code](https://github.com/tonsky/FiraCode) を試してみてください）。

### 入門

1. **Starship** のバイナリをインストール
  
    もし以下のプラットフォームを使用していない場合は **[コンパイル済みのバイナリファイルをダウンロード](https://github.com/starship/starship/releases)** してください。
  
  #### Homebrew

   ```sh
   $ brew install starship
   ```

#### Rust (v1.33 もしくはそれ以上)

   ```sh
   $ cargo install starship
   ```

#### Arch Linux (AUR)

Starship は AUR 上の `starship` というパッケージ名で利用可能です。 `yay` またはお好きな AUR ヘルパーでインストールしてください。

   ```sh
   $ yay -S starship
   ```

#### Nix (unstable)

   ```sh
   $ nix-env --install starship
   ```

#### Termux

   ```sh
   $ pkg install starship
   ```

1. 初期化のためのスクリプトをシェルの設定ファイルに追加
  
  #### Bash
  
    `~/.bashrc` の最後に以下を追記してください

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```

#### Fish

`~/.config/fish/config.fish` の最後に以下を追記してください

   ```sh
   # ~/.config/fish/config.fish

   eval (starship init fish)
   ```

#### Zsh

`~/.zshrc` の最後に以下を追記してください

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```

## 🔧 設定

Starship の設定方法の詳細に関しては、[ドキュメント](https://starship.rs/config/)をチェックしてください。

## 🤝 貢献

私たちは常に**すべてのスキルレベル**の貢献者を探しています！ もし簡単にプロジェクトへ参加する方法をお探しなら、 [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue) に取り組んでみてください。

### 特に必要としています

- 👩‍💼 **プロダクトマネージャー** 
  - GitHub のプロジェクトには、整理/優先度付けがされていない機能や、Issue になっていないアイデアが多くあります。 プロダクトの方針を持っていただける方が Starship には必要です！
- 👩‍🎨 **デザイナー** 
  - 目を引く Web サイトを作りたいですか？ 素晴らしい！ 我々は Starship の栄光を際立てる美しい Web サイトを作成したいと考えています。 Starship ブランドのデザインを手助けすることは、新しいアイデアを試す絶好の機会です！
- 👩‍💻 **Rust デベロッパー** 
  - 慣用的な Rust の作成、効果的な Rust アーキテクチャの設計、パフォーマンスの最適化、クロスプラットフォームビルドの最適化などに関しては、*多く*の簡単な成果があります。 私（[@matchai](https://github.com/matchai)）は Rust の初心者です。 私たちを正しい方向に向けてください！

もしあなたが Starship への貢献に興味がある場合は、我々の[貢献ガイド](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)をご覧ください。 また、気軽に我々の[Discord サーバー](https://discord.gg/8Jzqu3T)へ顔を出してください。 👋

### 貢献者

これらの素晴らしい方々に感謝します ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->

<!-- prettier-ignore -->

<table>
  <tr>
    <td align="center"><a href="https://twitter.com/matchai"><img src="https://avatars0.githubusercontent.com/u/4658208?v=4" width="100px;" alt="Matan Kushner"/><br /><sub><b>Matan Kushner</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=matchai" title="Code">💻</a> <a href="#design-matchai" title="Design">🎨</a> <a href="#ideas-matchai" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-matchai" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-matchai" title="Maintenance">🚧</a> <a href="#review-matchai" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=matchai" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/johnletey"><img src="https://avatars0.githubusercontent.com/u/30328854?v=4" width="100px;" alt="John Letey"/><br /><sub><b>John Letey</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=johnletey" title="Code">💻</a> <a href="#ideas-johnletey" title="Ideas, Planning, & Feedback">🤔</a> <a href="#review-johnletey" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=johnletey" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://timmulqueen.com"><img src="https://avatars1.githubusercontent.com/u/6132021?v=4" width="100px;" alt="Tim Mulqueen"/><br /><sub><b>Tim Mulqueen</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Multimo" title="Code">💻</a> <a href="#ideas-Multimo" title="Ideas, Planning, & Feedback">🤔</a> <a href="#review-Multimo" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=Multimo" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/sirMerr"><img src="https://avatars2.githubusercontent.com/u/11183523?v=4" width="100px;" alt="Tiffany Le-Nguyen"/><br /><sub><b>Tiffany Le-Nguyen</b></sub></a><br /><a href="#ideas-sirMerr" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-sirMerr" title="Maintenance">🚧</a> <a href="#review-sirMerr" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=sirMerr" title="Documentation">📖</a></td>
    <td align="center"><a href="https://about.snuggi.es"><img src="https://avatars0.githubusercontent.com/u/26250962?v=4" width="100px;" alt="​Snuggle"/><br /><sub><b>​Snuggle</b></sub></a><br /><a href="#design-Snuggle" title="Design">🎨</a> <a href="#ideas-Snuggle" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-Snuggle" title="Maintenance">🚧</a> <a href="#review-Snuggle" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/mehcode"><img src="https://avatars1.githubusercontent.com/u/753919?v=4" width="100px;" alt="Ryan Leckey"/><br /><sub><b>Ryan Leckey</b></sub></a><br /><a href="#review-mehcode" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/youssefhabri"><img src="https://avatars3.githubusercontent.com/u/1578005?v=4" width="100px;" alt="Youssef Habri"/><br /><sub><b>Youssef Habri</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=youssefhabri" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/chipbuster"><img src="https://avatars2.githubusercontent.com/u/4605384?v=4" width="100px;" alt="Kevin Song"/><br /><sub><b>Kevin Song</b></sub></a><br /><a href="https://github.com/starship/starship/issues?q=author%3Achipbuster" title="Bug reports">🐛</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=chipbuster" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://andrewda.me"><img src="https://avatars1.githubusercontent.com/u/10191084?v=4" width="100px;" alt="Andrew Dassonville"/><br /><sub><b>Andrew Dassonville</b></sub></a><br /><a href="https://github.com/starship/starship/issues?q=author%3Aandrewda" title="Bug reports">🐛</a> <a href="https://github.com/starship/starship/commits?author=andrewda" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/MaT1g3R"><img src="https://avatars1.githubusercontent.com/u/15258494?v=4" width="100px;" alt="MaT1g3R"/><br /><sub><b>MaT1g3R</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=MaT1g3R" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=MaT1g3R" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=MaT1g3R" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/AZanellato"><img src="https://avatars3.githubusercontent.com/u/30451287?v=4" width="100px;" alt="André Zanellato"/><br /><sub><b>André Zanellato</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=AZanellato" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=AZanellato" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=AZanellato" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://saghm.com"><img src="https://avatars2.githubusercontent.com/u/5875560?v=4" width="100px;" alt="Saghm Rossi"/><br /><sub><b>Saghm Rossi</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=saghm" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=saghm" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=saghm" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://medium.com/@cappyzawa"><img src="https://avatars3.githubusercontent.com/u/12455284?v=4" width="100px;" alt="Shu Kutsuzawa"/><br /><sub><b>Shu Kutsuzawa</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=cappyzawa" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=cappyzawa" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=cappyzawa" title="Tests">⚠️</a> <a href="#translation-cappyzawa" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/iamsauravsharma"><img src="https://avatars0.githubusercontent.com/u/38726015?v=4" width="100px;" alt="Saurav Sharma"/><br /><sub><b>Saurav Sharma</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=iamsauravsharma" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=iamsauravsharma" title="Documentation">📖</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/andytom"><img src="https://avatars1.githubusercontent.com/u/108836?v=4" width="100px;" alt="Thomas O'Donnell"/><br /><sub><b>Thomas O'Donnell</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=andytom" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=andytom" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/bbigras"><img src="https://avatars1.githubusercontent.com/u/24027?v=4" width="100px;" alt="Bruno Bigras"/><br /><sub><b>Bruno Bigras</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=bbigras" title="Code">💻</a> <a href="#review-bbigras" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://neilkistner.com/"><img src="https://avatars1.githubusercontent.com/u/186971?v=4" width="100px;" alt="Neil Kistner"/><br /><sub><b>Neil Kistner</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=wyze" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=wyze" title="Tests">⚠️</a> <a href="#review-wyze" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="http://ca.linkedin.com/in/qstrahl"><img src="https://avatars3.githubusercontent.com/u/2235277?v=4" width="100px;" alt="Quinn Strahl"/><br /><sub><b>Quinn Strahl</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=qstrahl" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=qstrahl" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/tivervac"><img src="https://avatars2.githubusercontent.com/u/3389524?v=4" width="100px;" alt="Titouan Vervack"/><br /><sub><b>Titouan Vervack</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=tivervac" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=tivervac" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://nosubstance.me"><img src="https://avatars1.githubusercontent.com/u/1269815?v=4" width="100px;" alt="Francisco Lopes"/><br /><sub><b>Francisco Lopes</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=oblitum" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ahouts"><img src="https://avatars1.githubusercontent.com/u/16907671?v=4" width="100px;" alt="Andrew Houts"/><br /><sub><b>Andrew Houts</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=ahouts" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=ahouts" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=ahouts" title="Tests">⚠️</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/nickwb"><img src="https://avatars2.githubusercontent.com/u/594211?v=4" width="100px;" alt="Nick Young"/><br /><sub><b>Nick Young</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=nickwb" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=nickwb" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=nickwb" title="Tests">⚠️</a> <a href="#review-nickwb" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/g2p"><img src="https://avatars1.githubusercontent.com/u/61678?v=4" width="100px;" alt="Gabriel de Perthuis"/><br /><sub><b>Gabriel de Perthuis</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=g2p" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Hofer-Julian"><img src="https://avatars1.githubusercontent.com/u/30049909?v=4" width="100px;" alt="Hofer-Julian"/><br /><sub><b>Hofer-Julian</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=Hofer-Julian" title="Documentation">📖</a></td>
    <td align="center"><a href="http://blog.unhappychoice.com"><img src="https://avatars3.githubusercontent.com/u/5608948?v=4" width="100px;" alt="Yuji Ueki"/><br /><sub><b>Yuji Ueki</b></sub></a><br /><a href="#content-unhappychoice" title="Content">🖋</a> <a href="#translation-unhappychoice" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/heyrict"><img src="https://avatars3.githubusercontent.com/u/25698503?v=4" width="100px;" alt="谢祯晖"/><br /><sub><b>谢祯晖</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=heyrict" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=heyrict" title="Documentation">📖</a> <a href="#translation-heyrict" title="Translation">🌍</a> <a href="#review-heyrict" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://twitter.com/bookun2851"><img src="https://avatars2.githubusercontent.com/u/10346162?v=4" width="100px;" alt="Kutsuzawa Ryo"/><br /><sub><b>Kutsuzawa Ryo</b></sub></a><br /><a href="#review-bookun" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/starship/starship/commits?author=bookun" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=bookun" title="Tests">⚠️</a> <a href="#translation-bookun" title="Translation">🌍</a></td>
    <td align="center"><a href="https://github.com/hdevalke"><img src="https://avatars1.githubusercontent.com/u/2261239?v=4" width="100px;" alt="hdevalke"/><br /><sub><b>hdevalke</b></sub></a><br /><a href="#ideas-hdevalke" title="Ideas, Planning, & Feedback">🤔</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/jakubclark"><img src="https://avatars0.githubusercontent.com/u/19486495?v=4" width="100px;" alt="Kuba Clark"/><br /><sub><b>Kuba Clark</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=jakubclark" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=jakubclark" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=jakubclark" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://breax.org"><img src="https://avatars2.githubusercontent.com/u/862483?v=4" width="100px;" alt="Gimbar"/><br /><sub><b>Gimbar</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=gimbar" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=gimbar" title="Tests">⚠️</a> <a href="https://github.com/starship/starship/commits?author=gimbar" title="Documentation">📖</a></td>
    <td align="center"><a href="http://tomhotston.net"><img src="https://avatars0.githubusercontent.com/u/22729355?v=4" width="100px;" alt="Tom Hotston"/><br /><sub><b>Tom Hotston</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=TomHotston" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=TomHotston" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/bijancn"><img src="https://avatars3.githubusercontent.com/u/2117164?v=4" width="100px;" alt="Bijan Chokoufe Nejad"/><br /><sub><b>Bijan Chokoufe Nejad</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=bijancn" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=bijancn" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/yuri1969"><img src="https://avatars3.githubusercontent.com/u/13468636?v=4" width="100px;" alt="yuri"/><br /><sub><b>yuri</b></sub></a><br /><a href="https://github.com/starship/starship/commits?author=yuri1969" title="Code">💻</a> <a href="https://github.com/starship/starship/commits?author=yuri1969" title="Documentation">📖</a> <a href="https://github.com/starship/starship/commits?author=yuri1969" title="Tests">⚠️</a></td>
  </tr>
</table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

このプロジェクトは [all-contributors](https://github.com/all-contributors/all-contributors) の仕様に従っています。 どんな種類の貢献でもお待ちしています！

## 影響を受けたプロダクト

よければStarship の作成に影響を与えた、これまでのプロジェクトをチェックしてください 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - 宇宙飛行士のための ZSH プロンプト。

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - 多くの shell に対応した JavaScript で書かれた robbyrussell テーマ。

- **[reujab/silver](https://github.com/reujab/silver)** - 多くの shell に対応しているカスタマイズ可能でアイコンを表示できる powerline のようなプロンプト。

<p align="center">
    <br >
    <img width="100" src="media/icon.png" alt="Starship rocket icon">
</p>

## 📝 ライセンス

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).  
このプロジェクトは [ISC](https://github.com/starship/starship/blob/master/LICENSE) でライセンスされています。