App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let left, right, renamed, deep;
		let source = $$props["source"];
		$: ({left, right, alias: renamed, nested: {deep}} = source);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$left", left))}-${$.escape($.store_get($$store_subs ??= {}, "$right", right))}-${$.escape(renamed)}-${$.escape(deep)}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { source });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
