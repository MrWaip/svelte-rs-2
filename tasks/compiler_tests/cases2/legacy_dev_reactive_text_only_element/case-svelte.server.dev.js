App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = "x";
		let count = 0;
		function tick() {
			title = "y";
			count = 1;
		}
		const $$tag = "div";
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.push(`${$.escape(title)}</p>`);
		$.pop_element();
		$$renderer.push(` <strong>`);
		$.push_element($$renderer, "strong", 12, 0);
		$$renderer.push(`${$.escape(count)}</strong>`);
		$.pop_element();
		$$renderer.push(` `);
		$.push_element($$renderer, $$tag, 13, 0);
		$.element($$renderer, $$tag, void 0, () => {
			$$renderer.push(`Dyn: ${$.escape(title)}`);
		});
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
