import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	const c = 1;
	$: {
		a;
		c;
	}
	$.bind_props($$props, {
		a,
		c
	});
}
