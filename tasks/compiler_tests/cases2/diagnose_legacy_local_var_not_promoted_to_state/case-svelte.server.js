import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	let inserted = false;
	function shouldShow() {
		if (inserted) {
			return false;
		}
		inserted = true;
		return true;
	}
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		if (shouldShow()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>${$.escape(item)}</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
