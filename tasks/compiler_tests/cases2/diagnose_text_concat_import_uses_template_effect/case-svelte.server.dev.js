App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { NAME } from "./lib";
import Other from "./Other.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`Hello ${$.escape(NAME)} world`);
		Other($$renderer, {});
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
