App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { realValue } from "./utils";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = { value: 0 };
		function process(input) {
			return realValue.transform(input);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`${$.escape(realValue.label)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
