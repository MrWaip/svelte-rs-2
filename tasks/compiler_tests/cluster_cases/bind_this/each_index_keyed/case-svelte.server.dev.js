App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = $.fallback($$props["data"], () => [{ id: "1" }], true);
		let refs = $.fallback($$props["refs"], () => [], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(data);
		for (let index = 0, $$length = each_array.length; index < $$length; index++) {
			let { id } = each_array[index];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 7, 1);
			$$renderer.push(`${$.escape(id)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			data,
			refs
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
