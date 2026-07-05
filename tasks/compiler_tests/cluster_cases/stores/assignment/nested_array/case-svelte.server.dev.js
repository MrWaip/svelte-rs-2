App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let a, b, c, d;
		const s = writable([[1, 2], [3, 4]]);
		$: [[a, b], [c, d]] = $.store_get($$store_subs ??= {}, "$s", s);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 7, 0);
		$$renderer.push(`${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
