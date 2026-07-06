import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.push(`<div><input${$.attr("value", foo)}/></div>`);
	$.bind_props($$props, { foo });
}
