App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			count2 = 0;
			#doubled = $.derived(() => this.#count * 2);
			inc() {
				this.#count += 1;
				this.count2 += 1;
			}
			get value() {
				return this.#count;
			}
			get doubled() {
				return this.#doubled();
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`${$.escape(c.value)} ${$.escape(c.count2)} ${$.escape(c.doubled)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
