import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { rest } = $$props;
		var a;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
		$$renderer.async([$$promises[1]], ($$renderer) => {
			$$renderer.push(`<div${$.attributes({ ...rest }, void 0, { one: a })}>`);
			$.push_element($$renderer, "div", 2, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
