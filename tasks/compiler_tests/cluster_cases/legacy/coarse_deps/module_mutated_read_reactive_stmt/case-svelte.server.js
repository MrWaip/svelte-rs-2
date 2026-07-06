import * as $ from "svelte/internal/server";
let count = 0;
export default function App($$renderer, $$props) {
	let doubled;
	function bump() {
		count = count + 1;
	}
	$: doubled = count;
	$$renderer.push(`<p>${$.escape(doubled)}</p>`);
	$.bind_props($$props, { bump });
}
