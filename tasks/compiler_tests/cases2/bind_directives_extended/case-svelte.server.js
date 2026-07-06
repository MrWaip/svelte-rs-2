import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "";
	let checked = false;
	let group = void 0;
	$$renderer.push(`<input${$.attr("value", value)}/> <input${$.attr("value", value)}/> <input type="checkbox"${$.attr("checked", checked, true)}/> <input type="checkbox"${$.attr("checked", checked, true)}/> <input/> <input/>`);
}
