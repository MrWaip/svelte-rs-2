import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let content = "<b>hello</b>";
	$$renderer.push(`<div>${$.html(content)}</div>`);
}
