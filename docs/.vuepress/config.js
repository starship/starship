module.exports = {
  title: 'Starship',
  description: 'The cross-shell prompt for astronauts ☄🌌️',
  themeConfig: {
    // displayAllHeaders: true, // Default: false
    sidebar: [
      '/',
      ['/guide/', 'Guide'],
      '/configuration/'
    ],
    repo: 'starship/starship',
    repoLabel: 'GitHub',
    // if your docs are not at the root of the repo:
    docsDir: 'docs',
    // defaults to false, set to true to enable
    editLinks: true,
    // custom text for edit link. Defaults to "Edit this page"
    editLinkText: 'Edit this page on GitHub'
  }
}
