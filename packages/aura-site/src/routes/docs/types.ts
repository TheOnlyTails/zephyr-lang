export type DocsMetadata = {
	title: string
	published?: boolean
}

export type DocsCategory = {
	title: string
}

export type DocsPage = {
	path: string
	category: string[]
} & DocsMetadata

export type DocsTree = {
	path: string
	pages: (DocsPage | DocsTree)[]
} & DocsCategory
