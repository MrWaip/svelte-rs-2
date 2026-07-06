App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let foo = writable(42);
		let cond = true;
		let x = $.fallback($$props["x"], () => cond ? $.store_get($$store_subs ??= {}, "$foo", foo) : 0, true);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.push(`${$.escape(x)}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
