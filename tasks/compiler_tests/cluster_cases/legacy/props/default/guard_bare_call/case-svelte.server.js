import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	function make() {
		return 1;
	}
	let x = $.fallback($$props["x"], make, true);
	$$renderer.push(`<p>${$.escape(x)}</p>`);
	$.bind_props($$props, { x });
}
