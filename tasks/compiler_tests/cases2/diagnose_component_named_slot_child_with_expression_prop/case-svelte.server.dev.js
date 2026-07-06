App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = $$props["value"];
		Outer($$renderer, { $$slots: { footer: ($$renderer) => {
			Inner($$renderer, {
				slot: "footer",
				x: value
			});
		} } });
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
