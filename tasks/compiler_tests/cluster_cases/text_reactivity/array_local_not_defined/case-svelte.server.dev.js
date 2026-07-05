App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let log = [];
		function add() {
			log.push(1);
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.push(`v ${$.escape(log)}</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`+</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
