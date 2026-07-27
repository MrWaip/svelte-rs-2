import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		async function compute(v) {
			return v * 2;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.push(`${$.escape(count)}</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`double</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
