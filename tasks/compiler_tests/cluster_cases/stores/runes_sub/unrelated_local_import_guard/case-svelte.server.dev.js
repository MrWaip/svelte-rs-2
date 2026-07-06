App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { helper } from "./store.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`${$.escape(a)} ${$.escape(helper)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
