import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items, b } = $$props;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let $$async0 = each_array[$$index];
			$$renderer.push(`<b>`);
			$.push_element($$renderer, "b", 3, 1);
			$$renderer.push(`${$.escape($$async0)}</b>`);
			$.pop_element();
			$$renderer.push(`<i>`);
			$.push_element($$renderer, "i", 3, 18);
			$$renderer.push(async () => $.escape((await $.save(b))()));
			$$renderer.push(`</i>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
