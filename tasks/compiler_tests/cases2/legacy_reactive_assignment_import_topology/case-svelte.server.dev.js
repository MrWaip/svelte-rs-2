App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import data from "./dep.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let total, doubled;
		function bump() {
			data.count += 1;
		}
		$: total = data.count;
		$: doubled = total * 2;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`${$.escape(doubled)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
