import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let visible;
	let n = $$props["n"];
	let total = 0;
	function bump() {
		total = total + 1;
	}
	$: visible = compute(total, n);
	$$renderer.push(`<button>${$.escape(visible)}</button>`);
	$.bind_props($$props, { n });
}
