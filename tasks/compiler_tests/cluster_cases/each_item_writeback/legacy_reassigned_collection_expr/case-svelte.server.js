import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(foo.bar);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let bar = each_array[$$index];
			$$renderer.push(`<input${$.attr("value", bar)}/>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { foo });
	});
}
