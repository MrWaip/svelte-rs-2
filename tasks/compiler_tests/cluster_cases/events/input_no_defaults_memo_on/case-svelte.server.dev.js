App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const bubbler = createBubbler();
		$$renderer.push(`<input type="text"/>`);
		$.push_element($$renderer, "input", 5, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
