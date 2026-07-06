App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Tooltip from "./Tooltip.svelte";
import Icon from "./Icon.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Tooltip($$renderer, { $$slots: { activator: ($$renderer) => {
			$.css_props($$renderer, true, { "--color": "red" }, () => {
				Icon($$renderer, { slot: "activator" });
			});
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
