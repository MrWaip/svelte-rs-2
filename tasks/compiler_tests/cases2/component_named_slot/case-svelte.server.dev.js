App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Widget($$renderer, { $$slots: { footer: ($$renderer) => {
			$$renderer.push(`<p slot="footer">`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`Footer</p>`);
			$.pop_element();
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
