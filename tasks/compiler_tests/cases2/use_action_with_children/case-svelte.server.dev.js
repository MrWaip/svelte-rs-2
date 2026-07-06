App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { config, value } = $$props;
		$$renderer.push(`<label>`);
		$.push_element($$renderer, "label", 6, 0);
		$$renderer.push(`<input type="checkbox"/>`);
		$.push_element($$renderer, "input", 7, 1);
		$.pop_element();
		$$renderer.push(` <span>`);
		$.push_element($$renderer, "span", 8, 1);
		$$renderer.push(`${$.escape(value)}</span>`);
		$.pop_element();
		$$renderer.push(`</label>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
