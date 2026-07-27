App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let f = 0;
		async function go() {
			[f = false || await Promise.resolve(6)] = [];
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`${$.escape(f)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
