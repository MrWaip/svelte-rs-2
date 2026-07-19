App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, { $$slots: {
			x: ($$renderer) => {
				$$renderer.push(`<div class="a" slot="x">`);
				$.push_element($$renderer, "div", 6, 1);
				$$renderer.push(`a</div>`);
				$.pop_element();
			},
			y: ($$renderer) => {
				$$renderer.push(`<div class="b svelte-1k5tp9w" slot="y">`);
				$.push_element($$renderer, "div", 7, 1);
				$$renderer.push(`b</div>`);
				$.pop_element();
			}
		} });
		$$renderer.push(`<!----> <div class="c svelte-1k5tp9w">`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`c</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
