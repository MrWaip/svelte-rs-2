App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let i = 0;
		let index = 0;
		function bump() {
			i++;
		}
		Comp($$renderer, { active: i === index });
		$$renderer.push(`<!----> <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`bump</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
