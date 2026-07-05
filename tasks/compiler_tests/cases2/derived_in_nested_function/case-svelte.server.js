import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = 1;
	let b = 2;
	function compute() {
		const sum = $.derived(() => a + b);
		return sum();
	}
	$.bind_props($$props, { compute });
}
