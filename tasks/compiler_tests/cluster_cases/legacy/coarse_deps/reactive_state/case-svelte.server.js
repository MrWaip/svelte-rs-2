import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	let s = 0;
	function inc() {
		s++;
	}
	$: {
		a;
		s;
	}
	$.bind_props($$props, { a });
}
