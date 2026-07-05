App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $$props["items"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
			let item = each_array[$$index_1];
			$$renderer.push(`<!--[-->`);
			const each_array_1 = $.ensure_array_like(item.kids);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let kid = each_array_1[$$index];
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 8, 8);
				$$renderer.push(`${$.escape(kid)}</p>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { items });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
