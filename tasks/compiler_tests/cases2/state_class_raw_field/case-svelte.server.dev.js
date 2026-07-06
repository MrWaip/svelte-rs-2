App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Store {
			data = null;
			update(val) {
				this.data = val;
			}
		}
		let s = new Store();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.push(`${$.escape(s.data)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
