App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const name = writable("world");
		$$renderer.push(`<textarea>`);
		$.push_element($$renderer, "textarea", 6, 0);
		const $$body = $.escape($.store_get($$store_subs ??= {}, "$name", name));
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.pop_element();
		$$renderer.push(` <div contenteditable="true">`);
		$.push_element($$renderer, "div", 7, 0);
		const $$body_1 = $.store_get($$store_subs ??= {}, "$name", name);
		if ($$body_1) {
			$$renderer.push(`${$$body_1}`);
		} else {}
		$$renderer.push(`</div>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { name });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
