import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 1;
		let a;
		var promises = $$renderer.run([async () => a = (await $.save(Promise.resolve(n)))()]);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(a)));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 7, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
