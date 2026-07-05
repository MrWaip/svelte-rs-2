App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p class="svelte-13q9pxc">`);
		$.push_element($$renderer, "p", 21, 0);
		$$renderer.push(`animated</p>`);
		$.pop_element();
		$$renderer.push(` <div class="svelte-13q9pxc">`);
		$.push_element($$renderer, "div", 22, 0);
		$$renderer.push(`also animated</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
