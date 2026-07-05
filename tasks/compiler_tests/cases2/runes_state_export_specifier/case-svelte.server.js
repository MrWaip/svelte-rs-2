import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = 0;
	$$renderer.push(`<p>${$.escape(count)}</p>`);
	$.bind_props($$props, { count });
}
