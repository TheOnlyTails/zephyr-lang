const presetEnv = require("postcss-preset-env")

module.exports = {
	plugins: [
		require("tailwindcss/nesting/")("postcss-nested"),
		require("tailwindcss"),
		presetEnv({
			features: {
				"nesting-rules": false,
			},
			browsers: "last 2 versions, >= 0.25%, not dead",
		}),
		require("cssnano"),
	],
}
