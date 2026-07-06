App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { act } from "./act";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let ref;
		let attrs = {};
		$$renderer.push(`<div${$.attributes({ ...attrs })}>`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
