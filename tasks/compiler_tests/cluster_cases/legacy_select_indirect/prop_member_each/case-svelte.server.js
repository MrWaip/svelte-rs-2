import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected = $$props["selected"];
		let tasks = $$props["tasks"];
		$$renderer.select({ value: selected }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(tasks);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let task = each_array[$$index];
				$$renderer.option({ value: task }, ($$renderer) => {
					$$renderer.push(`${$.escape(task.description)}`);
				});
			}
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(` <label><input type="checkbox"${$.attr("checked", selected.done, true)}/> ${$.escape(selected.description)}</label> <!--[-->`);
		const each_array_1 = $.ensure_array_like(tasks.filter((t) => !t.done));
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let task = each_array_1[$$index_1];
			$$renderer.push(`<p>${$.escape(task.description)}</p>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			selected,
			tasks
		});
	});
}
