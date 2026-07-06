App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p class="svelte-a7w5kd">`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.push(`plain</p>`);
		$.pop_element();
		$$renderer.push(` <button class="svelte-a7w5kd">`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`btn</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
