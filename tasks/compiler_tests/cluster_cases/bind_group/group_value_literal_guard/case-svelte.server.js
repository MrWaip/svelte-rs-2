import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.push(`<input type="radio"${$.attr("checked", foo === false, true)}${$.attr("value", false)}/> <input type="radio"${$.attr("checked", foo === true, true)}${$.attr("value", true)}/>`);
	$.bind_props($$props, { foo });
}
