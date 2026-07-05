App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { transform } from "./utils.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = 0;
		const fn = $.derived(() => transform(value));
		$$renderer.push(`<div class="output">`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`${$.escape(fn()(value))}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
