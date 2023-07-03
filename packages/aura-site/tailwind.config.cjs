// @ts-check

const colors = require("tailwindcss/colors")

/**
 * @type {import("tailwindcss").Config}
 */
module.exports = {
	content: ["./src/**/*.{svelte,js,ts}"],
	safelist: [
		"mr-1",
		"opacity-20",
		"hover:opacity-60",
		"text-base",
		"font-bold",
		"inline-block",
		"align-middle",
		"relative",
		"-mt-1",
	],
	plugins: [require("@tailwindcss/typography"), require("daisyui")],
	daisyui: {
		logs: false,
	},
	theme: {
		extend: {
			colors: {
				divider: colors.gray[700],
			},
			minHeight: {
				"no-navbar": "calc(100dvh - 4rem)",
			},
			height: {
				"no-navbar": "calc(100dvh - 4rem)",
			},
			maxHeight: {
				"no-navbar": "calc(100dvh - 4rem)",
			},
		},
	},
}
