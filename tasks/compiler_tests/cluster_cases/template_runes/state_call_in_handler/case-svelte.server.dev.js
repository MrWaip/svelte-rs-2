App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { SvelteSet } from "svelte/reactivity";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const set = new SvelteSet();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`add</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
