import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const filters = [{ data: [1] }, { data: null }];
	let modeData = filters[0];
	$$renderer.push(`<button>swap</button> <!--[-->`);
	const each_array = $.ensure_array_like(modeData.data ?? []);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let curtain = each_array[$$index];
		$$renderer.push(`<div>${$.escape(curtain)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
