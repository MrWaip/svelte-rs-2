import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $.fallback($$props["value"], "");
	$$renderer.push(`<textarea>`);
	const $$body = $.escape(value);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</textarea>`);
	$.bind_props($$props, { value });
}
