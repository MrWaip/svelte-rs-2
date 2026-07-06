App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const count = writable(0);
		$.prevent_snippet_stringification(foo);
		function foo($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<!---->${$.escape($.store_get($$store_subs ??= {}, "$count", count))}`);
		}
		foo($$renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
