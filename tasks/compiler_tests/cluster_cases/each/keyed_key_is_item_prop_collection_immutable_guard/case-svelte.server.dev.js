App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { keys, columns } = $$props;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(keys);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let key = each_array[$$index];
			const column = columns[key];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 6, 1);
			$$renderer.push(`${$.escape(column)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
