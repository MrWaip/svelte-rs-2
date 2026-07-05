App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $.fallback($$props["items"], () => [], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let [ ...rest] = each_array[$$index];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 2);
			$$renderer.push(`${$.escape(rest[0])}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { items });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
