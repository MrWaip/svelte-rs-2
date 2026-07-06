App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		Wrap($$renderer, { $$slots: {
			image: ($$renderer) => {
				Inner($$renderer, { slot: "image" });
			},
			action: ($$renderer) => {
				$$renderer.push(`<span slot="action">`);
				$.push_element($$renderer, "span", 7, 4);
				$$renderer.push(`0</span>`);
				$.pop_element();
			}
		} });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
