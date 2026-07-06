App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { createFormatter } from "./utils.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		let items = [];
		// member on call result: items.filter(Boolean).length
		let total = $.derived(() => items.filter(Boolean).length);
		// call on import: createFormatter()
		let fmt = createFormatter();
		// member on import: createFormatter.defaults
		let defaults = createFormatter.defaults;
		// member on prop: data.nested
		let nested = data.nested;
		// new expression
		let map = new Map();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 24, 0);
		$$renderer.push(`${$.escape(total())} ${$.escape(fmt)} ${$.escape(defaults)} ${$.escape(nested)} ${$.escape(map)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
