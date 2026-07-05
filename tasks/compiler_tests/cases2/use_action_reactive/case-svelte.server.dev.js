App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let config = "hello";
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`text</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
