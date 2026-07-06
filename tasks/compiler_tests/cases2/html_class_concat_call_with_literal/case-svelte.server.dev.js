App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { getProductName } from "./helpers";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div${$.attr_class(`x0${$.stringify(getProductName())}`)}>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`hi</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
