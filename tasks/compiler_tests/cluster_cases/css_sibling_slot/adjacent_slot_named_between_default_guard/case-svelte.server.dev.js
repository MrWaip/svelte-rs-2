App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$$renderer.push(`<div class="a svelte-1uoiiwh">`);
				$.push_element($$renderer, "div", 6, 1);
				$$renderer.push(`a</div>`);
				$.pop_element();
				$$renderer.push(` <div class="c svelte-1uoiiwh">`);
				$.push_element($$renderer, "div", 8, 1);
				$$renderer.push(`c</div>`);
				$.pop_element();
			}),
			$$slots: {
				default: true,
				wut: ($$renderer) => {
					$$renderer.push(`<div class="b" slot="wut">`);
					$.push_element($$renderer, "div", 7, 1);
					$$renderer.push(`b</div>`);
					$.pop_element();
				}
			}
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
