import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let html = "";
	$$renderer.push(`<div contenteditable="true">`);
	if (html) {
		$$renderer.push(`${html}`);
	} else {
		$$renderer.push(`text ${$.escape(html)}`);
	}
	$$renderer.push(`</div>`);
}
