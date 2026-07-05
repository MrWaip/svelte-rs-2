App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let items = $$props["items"];
		const { list } = items;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$list", list));
		for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
			let item = each_array[idx];
			Child($$renderer, { value: item });
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { items });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
