import { defineConfig } from "vite";
import { browserslistToTargets } from "lightningcss";

export default defineConfig({
	css: {
		lightningcss: {
			targets: browserslistToTargets(["defaults"]),
			drafts: {
				customMedia: true
			}
		}
	},
	build: {
		cssMinify: "lightningcss"
	}
});
