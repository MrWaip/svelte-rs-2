App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { pick } from "./pick";
import { count } from "./count";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let kind = $$props["kind"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(pick(kind, $.store_get($$store_subs ??= {}, "$count", count)));
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 9, 4);
			$$renderer.push(`${$.escape(item)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { kind });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
