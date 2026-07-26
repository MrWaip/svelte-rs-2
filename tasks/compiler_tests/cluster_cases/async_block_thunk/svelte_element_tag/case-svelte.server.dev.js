import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		const $$tag = (await $.save(delay(x)))() + "div";
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.child_block(async ($$renderer) => {
			$.push_element($$renderer, $$tag, 11, 0);
			$.element($$renderer, $$tag, void 0, () => {
				$$renderer.push(`content`);
			});
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
