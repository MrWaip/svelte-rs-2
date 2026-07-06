import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let ops = [{
		args: [{
			value: [],
			options: [{ value: "a" }]
		}],
		id: 1
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(ops);
	for (let $$index_2 = 0, $$length = each_array.length; $$index_2 < $$length; $$index_2++) {
		let { args, id } = each_array[$$index_2];
		$$renderer.push(`<!--[-->`);
		const each_array_1 = $.ensure_array_like(args);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let arg = each_array_1[$$index_1];
			$$renderer.push(`<!--[-->`);
			const each_array_2 = $.ensure_array_like(arg.options);
			for (let $$index = 0, $$length = each_array_2.length; $$index < $$length; $$index++) {
				let { value } = each_array_2[$$index];
				$$renderer.push(`<input type="checkbox"${$.attr("checked", arg.value.includes(value), true)}${$.attr("value", value)}/>`);
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
}
