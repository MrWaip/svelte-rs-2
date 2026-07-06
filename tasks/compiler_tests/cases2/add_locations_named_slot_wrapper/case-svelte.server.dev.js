App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = "x" } = $$props;
		Widget($$renderer, { $$slots: { footer: ($$renderer) => {
			$$renderer.push(`<div slot="footer">`);
			$.push_element($$renderer, "div", 7, 4);
			$$renderer.push(`Footer: ${$.escape(value)}</div>`);
			$.pop_element();
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
