App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let s = writable(0);
		function swap() {
			s = writable(1);
		}
		$$renderer.push(`<input${$.attr("value", $.store_get($$store_subs ??= {}, "$s", s))}/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`swap</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
