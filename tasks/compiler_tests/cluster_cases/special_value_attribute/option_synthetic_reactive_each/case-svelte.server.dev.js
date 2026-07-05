App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected = $$props["selected"];
		let items = $$props["items"];
		$$renderer.select({ value: selected }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(items);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				$$renderer.option({}, item);
			}
			$$renderer.push(`<!--]-->`);
		});
		$.bind_props($$props, {
			selected,
			items
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
