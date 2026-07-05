App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		let baz = $.fallback($$props["baz"], undefined);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.push(`${$.escape(foo)}${$.escape(baz)}</p>`);
		$.pop_element();
		$.bind_props($$props, {
			foo,
			baz
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
