App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rows = [{ name: "a" }, { name: "b" }];
		function add() {
			rows = [...rows, { name: "c" }];
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`add</button>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
			let row = each_array[idx];
			const label = row.name + idx;
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 13, 4);
			$$renderer.push(`${$.escape(label)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
