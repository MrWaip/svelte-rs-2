App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#a;
			#b = { val: -1 };
			#c;
			constructor() {
				this.#a ||= { val: 0 };
				this.#b &&= { val: 0 };
				this.#c ??= { val: 0 };
			}
			get a() {
				return this.#a?.val;
			}
		}
		const counter = new Counter();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 18, 0);
		$$renderer.push(`${$.escape(counter.a)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
