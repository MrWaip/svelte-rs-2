import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	function greet() {
		return "hi";
	}
	$$renderer.push(`<p>${$.escape(greet())}</p>`);
	$.bind_props($$props, { greet });
}
