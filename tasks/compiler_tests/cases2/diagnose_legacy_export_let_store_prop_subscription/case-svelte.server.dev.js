App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let phone;
		let challenge = $$props["challenge"];
		let error = $$props["error"];
		$: phone = $.store_get($$store_subs ??= {}, "$challenge", challenge)?.phone || "";
		$$renderer.push(`<input${$.attr("value", $.store_get($$store_subs ??= {}, "$error", error))}/>`);
		$.push_element($$renderer, "input", 9, 0);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`${$.escape(phone)}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			challenge,
			error
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
