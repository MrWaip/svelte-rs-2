App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let anchorRefs = {};
		let groups = [];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(groups);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let group = each_array[$$index];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 7, 1);
			$$renderer.push(`${$.escape(group.key)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
