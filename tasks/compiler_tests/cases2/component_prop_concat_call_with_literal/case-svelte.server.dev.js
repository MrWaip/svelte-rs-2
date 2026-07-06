App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { getProductName } from "./helpers";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Comp($$renderer, { title: `x0${$.stringify(getProductName())}` });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
