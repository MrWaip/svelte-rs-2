import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = { x: [1] }, $$array = $.to_array(tmp.x, 1), bar = $.fallback($$props["bar"], () => $$array[0], true);
	function inc() {
		bar++;
	}
	$$renderer.push(`<button>${$.escape(bar)}</button>`);
	$.bind_props($$props, { bar });
}
