import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let d = $.fallback($$props["d"], 1);
	function inc() {
		d++;
	}
	$$renderer.push(`<button>${$.escape(d)}</button>`);
	$.bind_props($$props, { d });
}
