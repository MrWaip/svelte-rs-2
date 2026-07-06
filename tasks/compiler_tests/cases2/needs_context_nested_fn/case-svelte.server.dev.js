App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { api } from "./api.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function doSomething() {
			api.call();
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`click</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
