App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { A } from "./a";
import { B } from "./b";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let y;
		let x = $$props["x"];
		$: y = (function() {
			switch (x) {
				case A.ONE: return B;
				default: return "";
			}
		})();
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 15, 0);
		$$renderer.push(`${$.escape(y)}</p>`);
		$.pop_element();
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
