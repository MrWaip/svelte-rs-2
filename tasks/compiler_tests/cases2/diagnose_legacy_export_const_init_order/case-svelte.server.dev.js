App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { setContext } from "svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		let local = 0;
		const getLocal = () => local;
		setContext("k", a);
		local = a;
		$.bind_props($$props, {
			a,
			getLocal
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
