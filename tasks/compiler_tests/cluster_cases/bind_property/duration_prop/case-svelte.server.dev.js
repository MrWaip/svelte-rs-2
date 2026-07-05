App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let d = $$props["d"];
		$$renderer.push(`<video>`);
		$.push_element($$renderer, "video", 4, 0);
		$$renderer.push(`</video>`);
		$.pop_element();
		$.bind_props($$props, { d });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
