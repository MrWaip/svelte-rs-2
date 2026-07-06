App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let files = $.fallback($$props["files"], undefined);
		$$renderer.push(`<input type="file" multiple=""/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		$.bind_props($$props, { files });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
