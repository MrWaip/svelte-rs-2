import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function getItems() {
		return [
			1,
			2,
			3
		];
	}
	$$renderer.push(`<!--[-->`);
	$$renderer.child_block(async ($$renderer) => {
		const each_array = $.ensure_array_like((await $.save(getItems()))());
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<p>${$.escape(item)}</p>`);
		}
	});
	$$renderer.push(`<!--]-->`);
}
