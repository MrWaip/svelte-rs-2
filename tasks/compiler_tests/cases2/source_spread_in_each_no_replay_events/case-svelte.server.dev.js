App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { sources } = $$props;
		$$renderer.push(`<picture>`);
		$.push_element($$renderer, "picture", 5, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(sources);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let source = each_array[$$index];
			$$renderer.push(`<source${$.attributes({ ...source })}/>`);
			$.push_element($$renderer, "source", 7, 8);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></picture>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
