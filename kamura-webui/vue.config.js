const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  transpileDependencies: true,
  lintOnSave: false,
  devServer: {
    port: 19999,
    allowedHosts: [
      '.malloc.fun'
    ],
    webSocketServer: false,
  },
})
