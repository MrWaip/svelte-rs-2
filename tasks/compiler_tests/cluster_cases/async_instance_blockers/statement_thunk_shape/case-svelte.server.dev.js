import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let gate = 0;
		let sink = 0;
		var loaded, single;
		var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => {
			void (sink = 1);
			void (sink = 2);
			single = gate + 1;
		}]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sink)));
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(single)));
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
