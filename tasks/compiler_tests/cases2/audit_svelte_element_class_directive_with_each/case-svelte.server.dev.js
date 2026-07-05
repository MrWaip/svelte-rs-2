App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		let items = [{
			id: 1,
			active: false
		}, {
			id: 2,
			active: true
		}];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$.validate_dynamic_element_tag(() => tag);
			$.validate_void_dynamic_element(() => tag);
			$.push_element($$renderer, tag, 7, 1);
			$.element($$renderer, tag, () => {
				$$renderer.push(`${$.attr_class("", void 0, { "active": item.active })}`);
			}, () => {
				$$renderer.push(`${$.escape(item.id)}`);
			});
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
