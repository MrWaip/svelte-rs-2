App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Toggle {
			"aria-pressed" = false;
			toggle() {
				this["aria-pressed"] = !this["aria-pressed"];
			}
		}
		const toggle = new Toggle();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`${$.escape(toggle["aria-pressed"])}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
