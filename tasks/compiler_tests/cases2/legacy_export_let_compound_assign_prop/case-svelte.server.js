import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $.fallback($$props["count"], 0);
	count -= 7;
	$$renderer.push(`<!---->${$.escape(count)}`);
	$.bind_props($$props, { count });
}
