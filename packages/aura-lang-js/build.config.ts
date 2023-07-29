import { defineBuildConfig } from "unbuild"

export default defineBuildConfig({
	entries: [
		{
			input: "src/index",
			name: "index",
		},
		{
			input: "src/parser/index",
			name: "parser",
		},
		{
			input: "src/interpreter/index",
			name: "interpreter",
		},
	],
	declaration: true,
	clean: true,
	rollup: {
		inlineDependencies: true,
		emitCJS: true,
		cjsBridge: true,
	},
})
