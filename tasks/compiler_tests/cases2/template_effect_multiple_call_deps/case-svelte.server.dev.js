App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { scale } from "./utils.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = [
			10,
			20,
			30
		];
		const x = $.derived(() => scale([0, data.length], [0, 100]));
		const y = $.derived(() => scale([0, 30], [100, 0]));
		$$renderer.push(`<polyline${$.attr("points", data.map((d, i) => [x()(i), y()(d)]).join(" "))}>`);
		$.push_element($$renderer, "polyline", 8, 0);
		$$renderer.push(`</polyline>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
