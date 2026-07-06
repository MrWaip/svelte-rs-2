App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected = $$props["selected"];
		let tasks = $$props["tasks"];
		$$renderer.select({ value: selected }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(tasks);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let task = each_array[$$index];
				$$renderer.option({ value: task }, ($$renderer) => {
					$.push_element($$renderer, "option", 9, 2);
					$$renderer.push(`${$.escape(task.description)}`);
					$.pop_element();
				});
			}
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(` <label>`);
		$.push_element($$renderer, "label", 13, 0);
		$$renderer.push(`<input type="checkbox"${$.attr("checked", selected.done, true)}/>`);
		$.push_element($$renderer, "input", 14, 1);
		$.pop_element();
		$$renderer.push(` ${$.escape(selected.description)}</label>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array_1 = $.ensure_array_like(tasks.filter((t) => !t.done));
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let task = each_array_1[$$index_1];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 18, 1);
			$$renderer.push(`${$.escape(task.description)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			selected,
			tasks
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
