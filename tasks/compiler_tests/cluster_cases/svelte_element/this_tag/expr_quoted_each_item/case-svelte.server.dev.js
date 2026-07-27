App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tags = ["div", "span"];
		function bump() {
			tags = ["p"];
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array = $.ensure_array_like(tags);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let t = each_array[$$index];
			$.validate_dynamic_element_tag(() => t);
			$.validate_void_dynamic_element(() => t);
			$.push_element($$renderer, t, 7, 1);
			$.element($$renderer, t, void 0, () => {
				$$renderer.push(`hello`);
			});
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
