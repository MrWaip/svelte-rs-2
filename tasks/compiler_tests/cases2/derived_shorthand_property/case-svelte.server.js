import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = 0;
	function getInfo() {
		const computed = $.derived(() => value * 2);
		return { computed: computed() };
	}
	$.bind_props($$props, { getInfo });
}
