import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let width = 0;
	let content = "";
	$$renderer.push(`<div contenteditable="">`);
	if (content) {
		$$renderer.push(`${content}`);
	} else {
		$$renderer.push(`editable`);
	}
	$$renderer.push(`</div>`);
}
