App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let prop = $$props["prop"];
		let local = 0;
		let out = 0;
		$: if (true) {
			local = 1;
			out = (prop || 0) + local;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.push(`${$.escape(out)}</p>`);
		$.pop_element();
		$.bind_props($$props, { prop });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
