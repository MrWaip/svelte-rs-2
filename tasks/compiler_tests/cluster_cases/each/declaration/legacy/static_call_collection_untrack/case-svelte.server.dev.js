App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(Array(10).fill(null));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 8, 1);
			$$renderer.push(`${$.escape(i)}${$.escape(x)}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
