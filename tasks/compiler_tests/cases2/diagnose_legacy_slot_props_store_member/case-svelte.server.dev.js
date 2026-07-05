App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const search = writable({ value: "" });
		const error = writable("");
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {
			value: $.store_get($$store_subs ??= {}, "$search", search).value,
			isInvalid: Boolean($.store_get($$store_subs ??= {}, "$error", error))
		}, null);
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
