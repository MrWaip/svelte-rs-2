import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = 0;
	function getInfo() {
		const computed = $derived(value * 2);
		return { computed };
	}
	var $$exports = { getInfo };
	return $.pop($$exports);
}
