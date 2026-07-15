App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { config } = $$props;
		const makeStore = $.derived(() => config.makeStore);
		const entries = $.derived(() => makeStore()());
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 6, 0);
		$$renderer.push(`${$.escape(entries().x)}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
