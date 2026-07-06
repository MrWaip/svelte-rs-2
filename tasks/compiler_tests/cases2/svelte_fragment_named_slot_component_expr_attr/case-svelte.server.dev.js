App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = $$props["value"];
		Outer($$renderer, { $$slots: { content: ($$renderer) => {
			{
				Inner($$renderer, { prop: value });
			}
		} } });
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
