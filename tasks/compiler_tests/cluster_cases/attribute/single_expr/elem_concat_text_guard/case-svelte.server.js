import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	$$renderer.push(`<input${$.attr("data-x", `a${$.stringify(value)}`)}/>`);
	$.bind_props($$props, { value });
}
