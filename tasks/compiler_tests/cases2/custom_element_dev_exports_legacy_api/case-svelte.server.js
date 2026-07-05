import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x = 0, $$slots, $$events, ...rest } = $$props;
	const VERSION = "1";
	function helper() {}
	$$renderer.push(`<p>${$.escape(x)}</p>`);
	$.bind_props($$props, {
		VERSION,
		helper
	});
}
