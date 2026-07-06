import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = 0;
	function bump() {
		count = count + 1;
	}
	$$renderer.push(`<p>${$.escape(count)}</p>`);
	$.bind_props($$props, { bump });
}
