App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = $.fallback($$props["count"], 3);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(Array.from({ length: count }));
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let _ = each_array[$$index];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 4);
			$$renderer.push(`</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { count });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
