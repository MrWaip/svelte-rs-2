App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(new Array(4).fill(null));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 4, 4);
			$$renderer.push(`${$.escape(i)}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
