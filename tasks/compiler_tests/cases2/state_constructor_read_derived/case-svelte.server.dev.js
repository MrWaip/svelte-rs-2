App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			#doubled = $.derived(() => this.#count * 2);
			constructor() {
				console.log(this.#doubled());
			}
		}
		let c = new Counter();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.push(`${$.escape(c.display)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
