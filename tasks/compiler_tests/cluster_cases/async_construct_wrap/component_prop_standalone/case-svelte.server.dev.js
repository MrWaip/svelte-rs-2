import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x } = $$props;
		function delay(value) {
			return Promise.resolve(value);
		}
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(delay(x)))();
			Child($$renderer, { value: $$0 });
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
