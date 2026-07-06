App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<select>`);
		$.push_element($$renderer, "select", 4, 0);
		$$renderer.push(`<selectedcontent>`);
		$.push_element($$renderer, "selectedcontent", 5, 1);
		$$renderer.push(`</selectedcontent>`);
		$.pop_element();
		$$renderer.push(`<!></select>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
