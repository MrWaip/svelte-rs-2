import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let ratings = [0, 1];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(ratings);
	for (let index = 0, $$length = each_array.length; index < $$length; index++) {
		let value = each_array[index];
		Child($$renderer, { onChange: (v) => ratings[index] = v });
	}
	$$renderer.push(`<!--]-->`);
}
