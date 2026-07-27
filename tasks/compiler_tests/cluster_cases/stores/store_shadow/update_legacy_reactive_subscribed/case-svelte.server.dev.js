App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let base = $.fallback($$props["base"], 0);
		let count = 0;
		$: count = base * 2;
		$: $.store_set(count, 1);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 8, 0);
		$$renderer.push(`increment</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { base });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
