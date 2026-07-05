import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let content = "<em>hello</em>";
	$$renderer.push(`${$.html(content)}`);
}
