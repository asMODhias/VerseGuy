module.exports = {
  stories: ['../src/**/*.stories.@(ts|tsx|js)'],
  addons: ['@storybook/addon-essentials'],
  framework: {
    name: '@storybook/react',
    options: {}
  },
  core: {
    builder: 'storybook-builder-vite'
  }
}
