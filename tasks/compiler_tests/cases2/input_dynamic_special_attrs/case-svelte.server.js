import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "";
	let checked = false;
	let selected = true;
	let disabled = false;
	let readonly = false;
	$$renderer.push(`<input${$.attr("value", value)}${$.attr("disabled", disabled, true)}/> <input${$.attr("value", value)}${$.attr("readonly", readonly, true)}/> <input type="checkbox"${$.attr("checked", checked, true)}/> `);
	$$renderer.option({ selected }, ($$renderer) => {
		$$renderer.push(`picked`);
	});
}
