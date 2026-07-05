import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let content = "<circle cx='5' cy='5' r='5'></circle>";
	$$renderer.push(`<svg><g></g>${$.html(content)}</svg>`);
}
