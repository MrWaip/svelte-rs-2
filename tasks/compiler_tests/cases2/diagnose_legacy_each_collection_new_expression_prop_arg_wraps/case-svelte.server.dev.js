App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let size = $$props["size"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(new Array(size).fill(null));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 8, 4);
			$$renderer.push(`${$.escape(i)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { size });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
