App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			#a = { val: 0 };
			#b = 0;
			mix() {
				this.#a ??= { val: 1 };
				this.#b += 1;
			}
			get a() {
				return this.#a?.val;
			}
			get b() {
				return this.#b;
			}
		}
		const box = new Box();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 19, 0);
		$$renderer.push(`${$.escape(box.a)} ${$.escape(box.b)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
