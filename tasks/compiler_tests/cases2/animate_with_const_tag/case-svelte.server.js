import * as $ from "svelte/internal/server";
import { flip } from "svelte/animate";
export default function App($$renderer) {
	let items = [{
		id: 1,
		name: "a"
	}, {
		id: 2,
		name: "b"
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		const label = item.name.toUpperCase();
		$$renderer.push(`<div>${$.escape(label)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
