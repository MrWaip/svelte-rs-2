import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { items, b } = $$props;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let $$async0 = each_array[$$index];
		$$renderer.push(`<b>${$.escape($$async0)}</b><i>`);
		$$renderer.push(async () => $.escape((await $.save(b))()));
		$$renderer.push(`</i>`);
	}
	$$renderer.push(`<!--]-->`);
}
