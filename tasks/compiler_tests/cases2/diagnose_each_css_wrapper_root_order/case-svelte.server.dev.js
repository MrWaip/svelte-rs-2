App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Row from "./Row.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rows = $$props["rows"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let row = each_array[$$index];
			$.css_props($$renderer, true, { "--tone": "red" }, () => {
				Row($$renderer, { $$slots: { label: ($$renderer) => {
					$$renderer.push(`<span slot="label">`);
					$.push_element($$renderer, "span", 9, 8);
					$$renderer.push(`${$.escape(row.title)}</span>`);
					$.pop_element();
				} } });
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
