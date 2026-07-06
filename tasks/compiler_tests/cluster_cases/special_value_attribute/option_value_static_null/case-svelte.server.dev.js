App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.select({ value: foo }, ($$renderer) => {
			$$renderer.option({
				value: null,
				disabled: true
			}, ($$renderer) => {
				$.push_element($$renderer, "option", 6, 1);
				$$renderer.push(`Select an option`);
				$.pop_element();
			});
		});
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
