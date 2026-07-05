App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#n = 0;
			bump() {
				this.#n += 1;
				this.#n <<= 2;
				this.#n >>>= 1;
				this.#n &= 6;
			}
			get n() {
				return this.#n;
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 17, 0);
		$$renderer.push(`${$.escape(c.n)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
