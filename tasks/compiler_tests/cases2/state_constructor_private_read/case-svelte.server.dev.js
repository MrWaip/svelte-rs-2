App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Timer {
			#elapsed = 0;
			tick() {
				this.#elapsed += 1;
			}
			get display() {
				return this.#elapsed;
			}
		}
		let t = new Timer();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 14, 0);
		$$renderer.push(`${$.escape(t.display)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
