App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Holder from "./Holder.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Holder($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { value: Inner }) => {
				Inner($$renderer, {});
			} }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
