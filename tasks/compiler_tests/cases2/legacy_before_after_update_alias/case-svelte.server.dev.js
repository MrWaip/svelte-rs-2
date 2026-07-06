App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { beforeUpdate as before, afterUpdate as after } from "svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		before(() => {
			console.log("before");
		});
		after(() => {
			console.log("after");
		});
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 13, 0);
		$$renderer.push(`hooks</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
