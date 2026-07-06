import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import { flip } from "svelte/animate";
export default function App($$renderer) {
	let items = [{
		id: 1,
		name: "a"
	}];
	var data, params;
	var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => params = data.params]);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<div>${$.escape(item.name)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
