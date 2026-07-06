App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let list = [{ rows: [{ data: [] }] }];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(list);
		for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
			let group = each_array[$$index_1];
			$$renderer.push(`<!--[-->`);
			const each_array_1 = $.ensure_array_like(group.rows);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let item = each_array_1[$$index];
				$$renderer.push(`<input type="checkbox"${$.attr("checked", item.data.includes("a"), true)} value="a"/>`);
				$.push_element($$renderer, "input", 8, 2);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
