import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	let y = $$props["y"];
	$$renderer.push(`<pre>${$.escape(JSON.stringify($$sanitized_props))}</pre>`);
	$.bind_props($$props, { y });
}
