App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let watcherA = void 0;
		if (watcherA) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$watcherA", watcherA))} <button>`);
			$.push_element($$renderer, "button", 8, 1);
			$$renderer.push(`remove</button>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 10, 1);
			$$renderer.push(`add</button>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
