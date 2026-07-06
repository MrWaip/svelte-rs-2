import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const count = 5;
	$$renderer.push(`<p>${$.escape(count)}</p>`);
	$.bind_props($$props, { count });
}
