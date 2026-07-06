App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { foo } from "lib";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = {};
		let c = "";
		$: obj.purpose = (c ? c : "") + foo(obj.type);
		$$renderer.push(`<input${$.attr("value", c)}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", obj.x)}/>`);
		$.push_element($$renderer, "input", 9, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
