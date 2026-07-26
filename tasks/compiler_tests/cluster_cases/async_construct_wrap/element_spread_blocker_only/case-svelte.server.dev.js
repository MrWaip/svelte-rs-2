import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function delay(value) {
			return Promise.resolve(value);
		}
		var attrs;
		var $$promises = $$renderer.run([async () => attrs = await delay({ title: "hi" })]);
		$$renderer.async([$$promises[0]], ($$renderer) => {
			$$renderer.push(`<div${$.attributes({ ...attrs })}>`);
			$.push_element($$renderer, "div", 9, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
