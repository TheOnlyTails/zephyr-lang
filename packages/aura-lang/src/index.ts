export const auraFileExt = "aura"

export type AuraOptions = {
	/**
	 * The path of the source folder relative to the config file.
	 * @default "src"
	 */
	src?: string
	/**
	 * The name of the Aura config file, indicates the root of the project.
	 * @default "aura.toml"
	 */
	config?: string
}

export const defaultOptions = {
	src: "src/",
	config: "aura.toml",
} satisfies AuraOptions
