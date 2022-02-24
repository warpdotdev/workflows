const path = require('path');

module.exports = {
  entry: './index.ts',
  devtool: 'inline-source-map',
  target: 'node',
  mode: "production",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
       {
        test: /\.ya?ml$/,
        type: 'json', // Required by Webpack v4
        use: 'yaml-loader'
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'warp-workflows.js',
    path: path.resolve(__dirname, 'dist'),
    library: {
      name: 'warp-workflows',
      type: 'umd',
    },
  },
}