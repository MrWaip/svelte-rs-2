import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let checked = $$props["checked"];
	$$renderer.push(`<input type="checkbox"${$.attr("checked", checked, true)}/>`);
	$.bind_props($$props, { checked });
}
