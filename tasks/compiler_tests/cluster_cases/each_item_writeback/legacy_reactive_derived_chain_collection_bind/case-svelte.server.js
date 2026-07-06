import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let done, filtered;
	let items = [{ done: false }];
	let filter = $.fallback($$props["filter"], "all");
	$: done = items.filter((i) => i.done);
	$: filtered = filter === "all" ? items : done;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(filtered);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("checked", item.done, true)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { filter });
}
