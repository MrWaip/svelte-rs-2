import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $.fallback($$props["count"], 0);
	$$renderer.push(`<!---->${$.escape(count = 42)}`);
	$.bind_props($$props, { count });
}
