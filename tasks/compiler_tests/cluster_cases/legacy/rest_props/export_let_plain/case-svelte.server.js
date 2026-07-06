import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["foo"]);
	let foo = $.fallback($$props["foo"], 1);
	$$renderer.push(`<div${$.attributes({ ...$$restProps })}>${$.escape(foo)}</div>`);
	$.bind_props($$props, { foo });
}
