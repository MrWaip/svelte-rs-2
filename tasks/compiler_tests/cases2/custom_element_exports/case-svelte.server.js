import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value = 0 } = $$props;
	function reset() {
		value = 0;
	}
	$$renderer.push(`<p>${$.escape(value)}</p>`);
	$.bind_props($$props, { reset });
}
