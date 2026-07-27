App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		function bump() {
			tag = "p";
		}
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$.push_element($$renderer, tag, 6, 0);
		$.element($$renderer, tag, void 0, () => {
			$$renderer.push(`hello`);
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
