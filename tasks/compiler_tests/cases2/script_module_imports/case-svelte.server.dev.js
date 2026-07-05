App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
const theme = writable("light");
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.push(`0</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
