import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let active = $$props["active"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like([
		1,
		2,
		3
	]);
	for (let index = 0, $$length = each_array.length; index < $$length; index++) {
		let _ = each_array[index];
		$$renderer.push(`<input type="radio"${$.attr("value", index)}${$.attr("checked", active === index, true)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { active });
}
