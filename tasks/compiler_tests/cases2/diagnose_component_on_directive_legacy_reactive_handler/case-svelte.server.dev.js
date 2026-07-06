App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let handler;
		let cond = $.fallback($$props["cond"], false);
		function a() {}
		function b() {}
		$: handler = cond ? a : b;
		Child($$renderer, {});
		$.bind_props($$props, { cond });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
