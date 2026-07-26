import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		async function* gen() {
			yield 1;
		}
		for await (const n of gen()) {
			count = n;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 13, 0);
		$$renderer.push(`${$.escape(count)}</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 14, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
