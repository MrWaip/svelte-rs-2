import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let a = 1;
	let b = 2;
	function compute() {
		const sum = $.derived(() => a + b);
		return $.get(sum);
	}
	var $$exports = { compute };
	return $.pop($$exports);
}
