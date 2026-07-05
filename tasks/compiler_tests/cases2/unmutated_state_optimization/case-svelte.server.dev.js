App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		let label = "hello";
		let items = [
			1,
			2,
			3
		];
		function increment() {
			count += 1;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`${$.escape(count)}</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.push(`hello</p>`);
		$.pop_element();
		$$renderer.push(` <ul>`);
		$.push_element($$renderer, "ul", 13, 0);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<li>`);
			$.push_element($$renderer, "li", 15, 8);
			$$renderer.push(`${$.escape(item)}</li>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></ul>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
