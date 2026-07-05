App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { tag = "div" } = $$props;
		let title = "hello";
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.push_element($$renderer, tag, 6, 0);
		$.element($$renderer, tag, () => {
			$$renderer.push(` class="first"`);
		}, () => {
			$$renderer.push(`First: hello`);
		});
		$.pop_element();
		$$renderer.push(` `);
		$.push_element($$renderer, tag, 10, 0);
		$.element($$renderer, tag, () => {
			$$renderer.push(` class="second"`);
		}, () => {
			$$renderer.push(`Second: hello`);
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
