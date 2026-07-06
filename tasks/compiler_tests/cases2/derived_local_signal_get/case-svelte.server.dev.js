App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function getValues() {
			const doubled = $.derived(() => x * 2);
			return {
				doubled: doubled(),
				get live() {
					return doubled();
				}
			};
		}
		$.bind_props($$props, { getValues });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
