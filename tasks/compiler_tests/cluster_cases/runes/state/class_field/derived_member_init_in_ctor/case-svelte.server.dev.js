App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#doubled;
			get doubled() {
				return this.#doubled();
			}
			set doubled(value) {
				this.#doubled(value);
			}
			#count;
			constructor(initial) {
				this.#count = initial;
				this.#doubled = $.derived(() => this.#count * 2);
			}
			increment = () => {
				this.#count++;
			};
		}
		const counter = new Counter(10);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 22, 0);
		$$renderer.push(`${$.escape(counter.doubled)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
