App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import { foo } from "lib";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const obj = writable({});
		let c = "";
		$: {
			c;
			foo;
			$.store_get($$store_subs ??= {}, "$obj", obj);
		}
		$$renderer.push(`<input${$.attr("value", c)}/>`);
		$.push_element($$renderer, "input", 13, 0);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
