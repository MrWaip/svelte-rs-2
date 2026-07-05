App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Store {
			#data = { n: 0 };
			get data() {
				return this.#data;
			}
			inc() {
				this.#data.n++;
			}
		}
		const store = new Store();
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 14, 0);
		$$renderer.push(`${$.escape(store.data.n)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
