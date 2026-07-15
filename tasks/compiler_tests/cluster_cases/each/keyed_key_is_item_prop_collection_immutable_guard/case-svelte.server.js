import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { keys, columns } = $$props;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(keys);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let key = each_array[$$index];
			const column = columns[key];
			$$renderer.push(`<div>${$.escape(column)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
