import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $.fallback($$props["count"], 0);
	$$renderer.push(`<button>${$.escape(count)}</button>`);
	$.bind_props($$props, { count });
}
