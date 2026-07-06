import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["checked"]);
	let checked = $.fallback($$props["checked"], false);
	function k() {}
	$$renderer.push(`<div${$.attributes({ ...$$restProps })}><input${$.attr("checked", checked, true)} type="checkbox"/></div>`);
	$.bind_props($$props, { checked });
}
