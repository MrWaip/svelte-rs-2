import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = 0;
	function getDoubled() {
		const doubled = $.derived(() => count * 2);
		return doubled();
	}
	$.bind_props($$props, { getDoubled });
}
