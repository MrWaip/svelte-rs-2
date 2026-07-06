import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let url = "/api";
		function outer() {
			async function inner() {
				let $$d = await $.async_derived(() => fetch(url).then((r) => r.json())), data = $.derived(() => $$d().data), meta = $.derived(() => $$d().meta);
				return 1;
			}
			return inner;
		}
		$.bind_props($$props, { outer });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
