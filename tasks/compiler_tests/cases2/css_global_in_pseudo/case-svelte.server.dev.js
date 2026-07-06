App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p class="svelte-1rk0dqc">`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.push(`content</p>`);
		$.pop_element();
		$$renderer.push(` <p class="bar svelte-1rk0dqc">`);
		$.push_element($$renderer, "p", 13, 0);
		$$renderer.push(`bar</p>`);
		$.pop_element();
		$$renderer.push(` <div class="svelte-1rk0dqc">`);
		$.push_element($$renderer, "div", 14, 0);
		$$renderer.push(`box</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
