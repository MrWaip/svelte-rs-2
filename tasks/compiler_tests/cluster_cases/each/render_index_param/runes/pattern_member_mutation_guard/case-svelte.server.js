import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ obj: { x: 0 } }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { obj } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(obj.x)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
