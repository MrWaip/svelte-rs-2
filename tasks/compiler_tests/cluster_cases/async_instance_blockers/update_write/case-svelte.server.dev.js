import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let gate = 0;
		let counter = 1;
		var loaded;
		var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => void counter++]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 8, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(counter)));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
