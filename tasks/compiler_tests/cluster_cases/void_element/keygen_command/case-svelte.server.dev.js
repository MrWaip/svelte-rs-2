App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<keygen/>`);
		$.push_element($$renderer, "keygen", 1, 0);
		$.pop_element();
		$$renderer.push(` <command/>`);
		$.push_element($$renderer, "command", 2, 0);
		$.pop_element();
		$$renderer.push(` <br/>`);
		$.push_element($$renderer, "br", 3, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
