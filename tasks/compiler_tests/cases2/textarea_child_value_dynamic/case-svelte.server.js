import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "hello";
	$$renderer.push(`<textarea>`);
	const $$body = $.escape(value);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</textarea>`);
}
