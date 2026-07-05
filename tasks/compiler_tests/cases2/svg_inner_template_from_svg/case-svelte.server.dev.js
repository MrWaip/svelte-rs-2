App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		$$renderer.push(`<svg>`);
		$.push_element($$renderer, "svg", 5, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<circle${$.attr("cx", item * 10)}${$.attr("cy", 10)}${$.attr("r", 5)}>`);
			$.push_element($$renderer, "circle", 7, 2);
			$$renderer.push(`</circle>`);
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
