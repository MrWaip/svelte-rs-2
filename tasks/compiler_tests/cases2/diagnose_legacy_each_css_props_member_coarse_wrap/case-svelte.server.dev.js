App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Cell from "./Cell.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rows = $$props["rows"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let row = each_array[$$index];
			$.css_props($$renderer, true, { "--tone": row.muted ? undefined : "accent" }, () => {
				Cell($$renderer, {});
			});
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { rows });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
