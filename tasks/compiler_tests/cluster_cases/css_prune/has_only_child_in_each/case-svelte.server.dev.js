App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [1];
		$$renderer.push(`<div class="a svelte-10ib7zp">`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let x = each_array[$$index];
			$$renderer.push(`<div class="b svelte-10ib7zp">`);
			$.push_element($$renderer, "div", 7, 2);
			$$renderer.push(`${$.escape(x)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
