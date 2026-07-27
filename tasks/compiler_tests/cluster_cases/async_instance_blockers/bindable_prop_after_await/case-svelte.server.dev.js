import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var value;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => ({value = "test"} = $$props)]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`update</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(value)));
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
