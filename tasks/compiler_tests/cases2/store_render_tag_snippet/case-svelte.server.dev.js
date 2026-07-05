App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { snippet } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let arg = 1;
		$.store_get($$store_subs ??= {}, "$snippet", snippet)($$renderer, arg);
		$$renderer.push(`<!---->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
