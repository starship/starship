module.exports = {
  title: 'Starship',
  description: 'The cross-shell prompt for astronauts ☄🌌️',
  head: [
    ['link', { rel: 'icon', href: '/icon.png' }]
  ],
  themeConfig: {
    logo: '/icon.png',
    sidebar: [
      '/',
      ['/guide/', 'Guide'],
      ['/config/', 'Configuration']
    ],
    nav: [
      { text: 'Configuration', link: '/config/' },
    ],
    // the GitHub repo path
    repo: 'starship/starship',
    // the label linking to the repo
    repoLabel: 'GitHub',
    // if your docs are not at the root of the repo:
    docsDir: 'docs',
    // defaults to false, set to true to enable
    editLinks: true,
    // custom text for edit link. Defaults to "Edit this page"
    editLinkText: 'Edit this page on GitHub'
  }
}
