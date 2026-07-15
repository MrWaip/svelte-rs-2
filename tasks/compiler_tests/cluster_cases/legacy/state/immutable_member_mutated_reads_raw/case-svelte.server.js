import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let visible;
	let n = $$props["n"];
	let cache = {};
	function bump(i) {
		cache[i] = i;
	}
	$: visible = compute(cache, n);
	$$renderer.push(`<button>${$.escape(visible)}</button>`);
	$.bind_props($$props, { n });
}
