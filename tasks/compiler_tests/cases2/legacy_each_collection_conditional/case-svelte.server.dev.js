App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let cond = $.fallback($$props["cond"], true);
		let a = $.fallback($$props["a"], () => [1], true);
		let b = $.fallback($$props["b"], () => [2], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(cond ? a : b);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 9, 4);
			$$renderer.push(`${$.escape(item)}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			cond,
			a,
			b
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
