import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "Tom";
	$$renderer.push(`<!---->&amp; Tom &lt;`);
}
