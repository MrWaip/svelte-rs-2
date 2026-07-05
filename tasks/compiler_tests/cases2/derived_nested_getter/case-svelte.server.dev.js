App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = 0;
		function makeAccessor() {
			const computed = $.derived(() => value + 1);
			return { get computed() {
				return computed();
			} };
		}
		$.bind_props($$props, { makeAccessor });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
