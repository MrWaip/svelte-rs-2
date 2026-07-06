App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { error, fallback } = $$props;
		let status = $.derived(() => error ? "error" : fallback);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 6, 0);
		$$renderer.push(`${$.escape(status())}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
