import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let checked = $.fallback($$props["checked"], false);
	function onChange() {}
	$$renderer.push(`<input${$.attr("checked", checked, true)} type="checkbox"/>`);
	$.bind_props($$props, { checked });
}
