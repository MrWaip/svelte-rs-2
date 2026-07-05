App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = $$props["name"];
		let typed = $.fallback($$props["typed"], null);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.push(`${$.escape(name)}</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`${$.escape(typed)}</p>`);
		$.pop_element();
		$.bind_props($$props, {
			name,
			typed
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
