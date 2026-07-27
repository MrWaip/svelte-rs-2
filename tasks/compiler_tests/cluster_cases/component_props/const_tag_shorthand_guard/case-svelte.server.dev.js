App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let live = 0;
		let plain = 7;
		function bump() {
			live++;
		}
		$.prevent_snippet_stringification(row);
		function row($$renderer) {
			$.validate_snippet_args($$renderer);
			const kLit = "x";
			const kLive = live + 1;
			const kCall = Math.random();
			Child($$renderer, {
				kLive,
				kCall,
				plain,
				eLit: kLit,
				eLive: kLive
			});
		}
		row($$renderer);
		$$renderer.push(`<!----> <button>`);
		$.push_element($$renderer, "button", 18, 0);
		$$renderer.push(`b</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
