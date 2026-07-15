App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rows = [];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let _ = 0, $$length = each_array.length; _ < $$length; _++) {
			let row = each_array[_];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`${$.escape(row.name)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_1 = $.ensure_array_like(rows);
		for (let i_dx = 0, $$length = each_array_1.length; i_dx < $$length; i_dx++) {
			let row = each_array_1[i_dx];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 10, 1);
			$$renderer.push(`${$.escape(row.name)}${$.escape(i_dx)}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
