App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Inner($$renderer, { $$slots: {
			a: ($$renderer) => {
				$$renderer.push(`<div slot="a">`);
				$.push_element($$renderer, "div", 6, 4);
				$$renderer.push(`</div>`);
				$.pop_element();
			},
			b: ($$renderer) => {
				$$renderer.push(`<div slot="b">`);
				$.push_element($$renderer, "div", 7, 4);
				$$renderer.push(`</div>`);
				$.pop_element();
			},
			c: ($$renderer) => {
				$$renderer.push(`<div slot="c">`);
				$.push_element($$renderer, "div", 8, 4);
				$$renderer.push(`x</div>`);
				$.pop_element();
			}
		} });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
