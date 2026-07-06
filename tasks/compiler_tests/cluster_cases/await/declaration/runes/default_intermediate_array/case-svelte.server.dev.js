App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let p = Promise.resolve([[1, 2], 3]);
		$.await($$renderer, p, () => {}, ([[a, b] = [8, 9], c]) => {
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 5, 1);
			$$renderer.push(`${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
