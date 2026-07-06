App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const value = writable({
			a: 1,
			b: 2
		});
		if ($.store_get($$store_subs ??= {}, "$value", value)) {
			$$renderer.push("<!--[0-->");
			const { a, b } = $.store_get($$store_subs ??= {}, "$value", value);
			Comp($$renderer, {
				x: a,
				b
			});
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
