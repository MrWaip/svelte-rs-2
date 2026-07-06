App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const BASE = "https://example.com";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let path = "/home";
		let url = $.derived(() => BASE + path);
		$$renderer.push(`<a${$.attr("href", url())}>`);
		$.push_element($$renderer, "a", 10, 0);
		$$renderer.push(`Link</a>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
