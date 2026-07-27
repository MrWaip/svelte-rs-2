import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		var delayed, doubled;
		var $$promises = $$renderer.run([async () => delayed = await $.async_derived(() => Promise.resolve(count)), () => doubled = $.derived(() => count * 2)]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 7, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(delayed())));
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(doubled !== doubled())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
