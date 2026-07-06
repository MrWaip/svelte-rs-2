App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { items } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$items", items));
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`${$.escape(item)}</p>`);
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
