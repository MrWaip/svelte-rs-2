App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items = [] } = $$props;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let [id, name] = each_array[$$index];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`${$.escape(name)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let { id, name } = each_array_1[$$index_1];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 10, 1);
			$$renderer.push(`${$.escape(name)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_2 = $.ensure_array_like(items);
		for (let idx = 0, $$length = each_array_2.length; idx < $$length; idx++) {
			let [id, name] = each_array_2[idx];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 14, 1);
			$$renderer.push(`${$.escape(idx)}: ${$.escape(name)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_3 = $.ensure_array_like(items);
		for (let idx = 0, $$length = each_array_3.length; idx < $$length; idx++) {
			let [a, b, c] = each_array_3[idx];
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_4 = $.ensure_array_like(items);
		for (let idx = 0, $$length = each_array_4.length; idx < $$length; idx++) {
			let { a, b, c } = each_array_4[idx];
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
