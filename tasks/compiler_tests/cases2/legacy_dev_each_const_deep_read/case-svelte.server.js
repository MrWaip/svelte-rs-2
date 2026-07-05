import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let rows = [{ name: "a" }, { name: "b" }];
	function add() {
		rows = [...rows, { name: "c" }];
	}
	$$renderer.push(`<button>add</button> <!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
		let row = each_array[idx];
		const label = row.name + idx;
		$$renderer.push(`<p>${$.escape(label)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
