App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export const theme = writable("light");
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`${$.escape(count)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
