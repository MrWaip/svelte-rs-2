App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let selected = $$props["selected"];
		$$renderer.select({
			multiple: true,
			value: selected
		}, ($$renderer) => {
			$$renderer.option({}, ($$renderer) => {
				$.push_element($$renderer, "option", 6, 1);
				$$renderer.push(`a`);
				$.pop_element();
			});
			$$renderer.option({}, ($$renderer) => {
				$.push_element($$renderer, "option", 7, 1);
				$$renderer.push(`b`);
				$.pop_element();
			});
		});
		$.bind_props($$props, { selected });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
