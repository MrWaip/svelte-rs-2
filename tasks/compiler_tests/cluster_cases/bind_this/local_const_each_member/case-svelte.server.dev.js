App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const items1 = {};
		let data = [{
			id: 1,
			text: "a"
		}];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(data);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 7, 1);
			$$renderer.push(`${$.escape(item.text)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
