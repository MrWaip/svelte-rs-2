App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.option({ value: foo }, ($$renderer) => {
			$.push_element($$renderer, "option", 5, 0);
			$$renderer.push(`${$.escape(foo)}`);
			$.pop_element();
		});
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
