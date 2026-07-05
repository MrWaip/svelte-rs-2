import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["bar"]);
	let foo = $.fallback($$props["bar"], 1);
	$$renderer.push(`<div${$.attributes({ ...$$restProps })}>${$.escape(foo)}</div>`);
	$.bind_props($$props, { bar: foo });
}
