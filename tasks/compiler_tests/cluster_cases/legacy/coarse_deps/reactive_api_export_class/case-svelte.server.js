import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	class K {}
	$: {
		a;
		K;
	}
	$.bind_props($$props, {
		a,
		K
	});
}
