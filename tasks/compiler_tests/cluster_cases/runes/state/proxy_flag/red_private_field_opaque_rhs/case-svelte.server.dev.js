App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			get count() {
				return this.#count;
			}
			set count(val) {
				this.#count = val;
			}
		}
		const counter = new Counter();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 14, 0);
		$$renderer.push(`${$.escape(counter.count)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
