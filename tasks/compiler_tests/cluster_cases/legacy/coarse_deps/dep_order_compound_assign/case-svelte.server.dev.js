App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { foo } from "lib";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let c = 0;
		$: c += foo();
		$$renderer.push(`<input${$.attr("value", c)}/>`);
		$.push_element($$renderer, "input", 7, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
