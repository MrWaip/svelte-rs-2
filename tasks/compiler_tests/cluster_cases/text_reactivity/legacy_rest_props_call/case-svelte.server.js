import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["y"]);
	let x = $$props["y"];
	$$renderer.push(`<pre>${$.escape(JSON.stringify($$restProps))}</pre>`);
	$.bind_props($$props, { y: x });
}
