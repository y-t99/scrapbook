const path = require('path');

module.exports = {
    entry: './src-ui/index.js',
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist'),
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
            {
                test: /\.(woff|woff2|eot|ttf|otf)$/i,
                type: 'asset/resource',
                generator: {
                    filename: 'fonts/[hash][ext][query]'
                }
            },
        ],
    },
    performance: {
        assetFilter: function (assetFilename) {
            return !/\.(woff|woff2|eot|ttf|otf)$/.test(assetFilename);
        },
    },
};
