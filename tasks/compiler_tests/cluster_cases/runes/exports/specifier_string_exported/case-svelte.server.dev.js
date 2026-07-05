App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = 1;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.push(`${$.escape(foo)}</p>`);
		$.pop_element();
		$.bind_props($$props, { "foo-bar": foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
