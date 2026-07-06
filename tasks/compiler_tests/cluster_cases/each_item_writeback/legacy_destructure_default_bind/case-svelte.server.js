import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let array = $.fallback($$props["array"], () => [{ value: "" }, {}], true);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(array);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { value = "hello" } = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", value)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { array });
}
