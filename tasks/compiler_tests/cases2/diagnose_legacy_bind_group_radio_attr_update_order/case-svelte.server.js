import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let model = $.fallback($$props["model"], "a");
	let value = $.fallback($$props["value"], "a");
	let name = $.fallback($$props["name"], "radio");
	let hasError = $.fallback($$props["hasError"], false);
	$$renderer.push(`<input type="radio"${$.attr("value", value)}${$.attr("name", name)}${$.attr("checked", model === value, true)}${$.attr_class("", void 0, { "error": hasError })}/>`);
	$.bind_props($$props, {
		model,
		value,
		name,
		hasError
	});
}
