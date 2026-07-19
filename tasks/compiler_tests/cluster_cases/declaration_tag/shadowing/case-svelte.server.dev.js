App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value } = $$props;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like([1, 2]);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let n = each_array[$$index];
			const value = n * 10;
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`${$.escape(value)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <span>`);
		$.push_element($$renderer, "span", 8, 0);
		$$renderer.push(`${$.escape(value)}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
