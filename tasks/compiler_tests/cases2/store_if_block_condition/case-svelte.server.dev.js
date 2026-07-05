App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { cond } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		if ($.store_get($$store_subs ??= {}, "$cond", cond)) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 5, 11);
			$$renderer.push(`visible</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
