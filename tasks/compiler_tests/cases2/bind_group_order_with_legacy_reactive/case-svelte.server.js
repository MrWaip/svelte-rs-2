import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let id;
	let value = $$props["value"];
	let group = $$props["group"];
	$: id = `${value}-radio`;
	$$renderer.push(`<input type="radio"${$.attr("checked", group === value, true)}${$.attr("id", id)}${$.attr("value", value)}/>`);
	$.bind_props($$props, {
		value,
		group
	});
}
