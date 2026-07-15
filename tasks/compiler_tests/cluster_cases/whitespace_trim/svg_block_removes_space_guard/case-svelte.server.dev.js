App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items } = $$props;
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 5, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let i = each_array[$$index];
			$$renderer.push(`<rect>`);
			$.push_element($$renderer, "rect", 7, 2);
			$$renderer.push(`</rect>`);
			$.pop_element();
			$$renderer.push(`<rect>`);
			$.push_element($$renderer, "rect", 8, 2);
			$$renderer.push(`</rect>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
