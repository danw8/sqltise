module.exports = {
  pages:{
    index:{
      entry:'src/main.js',
    }
  },
  publicPath: process.env.NODE_ENV === 'production'
    ? 'sqltise'
    : '',
  outputDir: '../docs/'
}