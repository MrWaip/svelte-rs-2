App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { onMount } from "svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let derived_count;
		let flag = false;
		onMount(() => {
			const cb = () => {
				flag = true;
			};
			cb();
		});
		$: derived_count = (() => {
			if (flag) {
				return 1;
			}
			return 0;
		})();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 15, 0);
		$$renderer.push(`${$.escape(derived_count)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
