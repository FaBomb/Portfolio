const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");

const BUILD_MODE = "BUILD_MODE";
const release = !["dev"].includes(process.env[BUILD_MODE]);

module.exports = {
    mode: release ?  "production" : "development",
    entry: {
        app: path.resolve(__dirname, "index.ts"),
    },
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "[name].[contenthash].js",
        assetModuleFilename: "images/[name][ext]",
        publicPath: '/'
    },
    module: {
        rules: [
            {
                test: [/\.js$/, /\.ts$/],
                exclude: /node_modules/,
                loader: "babel-loader",
                options: {
                    presets: ["@babel/preset-typescript"],
                }
            },
            {
				test: /\.scss$/i,
				use: [
					{
						loader: 'style-loader',
					},
					{
						loader: 'css-loader',
					},
					{
						loader: 'sass-loader',
						options: {
							sassOptions: {
								outputStyle: 'expanded',
							},
						},
					},
				]
			},
            {
                test: /\.(gif|png|jpg|svg|ico)$/,
                type: "asset/resource",
            },
        ],
    },
    resolve: {
        extensions: [".js", ".jsx", ".ts", ".tsx"],
    },
    experiments: {
        asyncWebAssembly: true,
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "index.ejs"),
            filename: "index.html",
            inject: false,
            templateParameters(_a,_b,tags) {
                return {
                    scripts: tags.headTags
                }
            }
        }),
        new webpack.EnvironmentPlugin({
            [BUILD_MODE]: null,
            FIREBASE_CONFIG: JSON.stringify(require("./firebase.config.dev.json"))
        })
    ],
    performance: {
        maxEntrypointSize: 500000,
        maxAssetSize: 500000,
    },
};