App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const fn = (node, options) => ({});
		let a = { b: { "c-d": fn } };
		let directive = $.derived(() => a);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like([]);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let i = each_array[$$index];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 8, 1);
			$$renderer.push(`</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
