import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	class Counter {}
	$$renderer.push(`<p>ok</p>`);
	$.bind_props($$props, { Counter });
}
