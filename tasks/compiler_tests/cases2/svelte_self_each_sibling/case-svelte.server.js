import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [1, 2];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<p>ok</p> `);
		App($$renderer, { value: item });
		$$renderer.push(`<!---->`);
	}
	$$renderer.push(`<!--]-->`);
}
