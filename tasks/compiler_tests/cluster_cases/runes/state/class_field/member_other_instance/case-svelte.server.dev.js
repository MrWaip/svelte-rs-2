App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			#value;
			constructor(value) {
				this.#value = value;
			}
			get value() {
				return this.#value;
			}
			swap(other) {
				const value = this.#value;
				this.#value = other.value;
				other.#value = value;
			}
		}
		const a = new Box(42);
		const b = new Box(99);
		a.swap(b);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
