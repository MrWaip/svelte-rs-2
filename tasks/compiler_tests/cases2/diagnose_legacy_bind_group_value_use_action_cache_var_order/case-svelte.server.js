import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let model = $.fallback($$props["model"], "a");
	let value = $.fallback($$props["value"], "a");
	function action() {}
	$$renderer.push(`<input type="radio"${$.attr("value", value)}${$.attr("checked", model === value, true)}/>`);
	$.bind_props($$props, {
		model,
		value
	});
}
