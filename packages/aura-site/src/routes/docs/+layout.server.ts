import { error } from "@sveltejs/kit"
import type { DocsCategory, DocsMetadata, DocsPage, DocsTree } from "./types"
import type { LayoutServerLoad } from "./$types"

const pagePathTrim = /\.((\/[\w-]+)+)\/\+page\.md/
const categoryPathTrim = /\.((\/[\w-]+)+)\/category\.ts/

const getPages = () => {
	const rawPages = import.meta.glob<DocsMetadata>(
		["./**/*/+page.md", "./+page.md"], // import all docs pages + base page
		{ eager: true, import: "metadata" }
	)

	return Object.entries(rawPages).map(([path, page]) => {
		const pagePath = path.match(pagePathTrim)?.[1] ?? ""
		return {
			path: pagePath,
			category: pagePath.split("/").slice(1, -1),
			...page,
		} as DocsPage
	})
}

const getTree = (pages: DocsPage[]) => {
	const rawCategories = import.meta.glob<DocsCategory>("./**/*/category.ts", {
		eager: true,
		import: "category",
	})

	const categories: DocsTree[] = Object.entries(rawCategories).map(([path, category]) => {
		const categoryPath = path.match(categoryPathTrim)?.[1] ?? ""
		const categoryArray = categoryPath.split("/").slice(1)

		return {
			pages: pages.filter((page) => categoryArray.every((p, i) => p === page.category[i])),
			path: categoryPath,
			...category,
		}
	})

	return [...pages.filter((page) => page.category.length === 0), ...categories] as (
		| DocsPage
		| DocsTree
	)[]
}

const docsPages = getPages()
const docsTree = getTree(docsPages)

export const load: LayoutServerLoad = async ({ route, url }) => {
	const currentPage = docsPages.find((page) => "/docs" + page.path === route.id)
	if (!currentPage) throw error(404, `Couldn't find a docs page in ${url}.`)

	return {
		tree: docsTree,
		currentPage,
	}
}
