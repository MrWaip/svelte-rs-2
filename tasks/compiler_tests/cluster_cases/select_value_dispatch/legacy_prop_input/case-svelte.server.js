import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let name = $$props["name"];
	$$renderer.push(`<input${$.attr("value", name)}/>`);
	$.bind_props($$props, { name });
}
