App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			count = 0;
			#double = $.derived(() => this.count * 2);
			get double() {
				return this.#double();
			}
			set double($$value) {
				return this.#double($$value);
			}
			inc() {
				this.count += 1;
			}
			get viaAlias() {
				const self = this;
				return self.count;
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`${$.escape(c.count)} ${$.escape(c.double)} ${$.escape(c.viaAlias)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
