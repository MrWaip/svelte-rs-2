import * as $ from "svelte/internal/server";
let count = 0;
export default function App($$renderer, $$props) {
	function bump() {
		count = count + 1;
	}
	$$renderer.push(`<p>${$.escape(count)}</p>`);
	$.bind_props($$props, { bump });
}
