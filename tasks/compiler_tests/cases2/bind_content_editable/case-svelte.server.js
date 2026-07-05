import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let html = "";
	let text = "";
	let content = "";
	$$renderer.push(`<div contenteditable="">`);
	if (html) {
		$$renderer.push(`${html}`);
	} else {}
	$$renderer.push(`</div> <div contenteditable="">`);
	const $$body = $.escape(text);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</div> <div contenteditable="">`);
	const $$body_1 = $.escape(content);
	if ($$body_1) {
		$$renderer.push(`${$$body_1}`);
	} else {}
	$$renderer.push(`</div>`);
}
