App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let arr = [
			1,
			2,
			3
		];
		let elements = [];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(arr);
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let item = each_array[i];
			$$renderer.push(`<b>`);
			$.push_element($$renderer, "b", 6, 1);
			$$renderer.push(`${$.escape(item)}</b>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
