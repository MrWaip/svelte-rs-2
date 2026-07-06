import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { v = "" } = $$props;
	$$renderer.push(`<textarea>`);
	const $$body = $.escape(v);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</textarea>`);
}
