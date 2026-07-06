import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	const k = 1;
	$: {
		a;
		k;
	}
	$.bind_props($$props, { a });
}
