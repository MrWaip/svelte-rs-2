import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let people = $$props["people"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(people);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { name: { first: f, last: l } } = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", f)}/> <input${$.attr("value", l)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { people });
}
