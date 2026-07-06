App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			#total;
			get total() {
				return this.#total();
			}
			set total($$value) {
				return this.#total($$value);
			}
			width = 2;
			height = 3;
			#area = $.derived(() => this.width * this.height);
			get area() {
				return this.#area();
			}
			set area($$value) {
				return this.#area($$value);
			}
			constructor() {
				this.#total = $.derived(() => this.area + 1);
			}
		}
		let box = new Box();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 15, 0);
		$$renderer.push(`${$.escape(box.area)},${$.escape(box.total)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
