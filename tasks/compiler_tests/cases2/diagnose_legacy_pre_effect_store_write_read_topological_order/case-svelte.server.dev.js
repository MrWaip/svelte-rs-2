App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const w = writable(0);
		let derived = 0;
		$: (() => {
			$.store_set(w, 1);
		})();
		$: derived = $.store_get($$store_subs ??= {}, "$w", w) * 2;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.push(`${$.escape(derived)}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
