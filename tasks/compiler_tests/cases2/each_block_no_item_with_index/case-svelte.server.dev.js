App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let rank = 0, $$length = each_array.length; rank < $$length; rank++) {
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 6, 1);
			$$renderer.push(`${$.escape(rank)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
