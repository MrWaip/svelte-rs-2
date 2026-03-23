import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = 0;
	function getDoubled() {
		const doubled = $.derived(() => count * 2);
		return $.get(doubled);
	}
	var $$exports = { getDoubled };
	return $.pop($$exports);
}
