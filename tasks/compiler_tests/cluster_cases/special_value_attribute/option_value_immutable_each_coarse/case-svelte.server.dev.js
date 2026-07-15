App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $$props["items"];
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 6, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.option({ value: item.value }, ($$renderer) => {
				$.push_element($$renderer, "option", 8, 2);
				$$renderer.push(`${$.escape(item.text)}`);
				$.pop_element();
			});
		}
		$$renderer.push(`<!--]--></select>`);
		$.pop_element();
		$.bind_props($$props, { items });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
