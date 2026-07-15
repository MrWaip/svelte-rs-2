import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let anchorRefs = {};
	let groups = [];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(groups);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let group = each_array[$$index];
		$$renderer.push(`<div>${$.escape(group.key)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
