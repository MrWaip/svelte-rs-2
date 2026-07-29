App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			increment() {
				this.#count += 1;
			}
		}
		const counter = new Counter();
		function handleError(e) {
			console.error(e, counter);
		}
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<!---->x`);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
