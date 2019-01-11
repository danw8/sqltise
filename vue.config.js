module.exports = {
  pages:{
    index:{
      entry:'ui/src/main.js',
    }
  },
  publicPath: process.env.NODE_ENV === 'production'
    ? ''
    : ''
}