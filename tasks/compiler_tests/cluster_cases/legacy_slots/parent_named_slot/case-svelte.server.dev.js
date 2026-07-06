App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		A($$renderer, { $$slots: { s: ($$renderer) => {
			$$renderer.push(`<span slot="s">`);
			$.push_element($$renderer, "span", 6, 1);
			$$renderer.push(`x</span>`);
			$.pop_element();
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
