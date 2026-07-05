App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { inputValue = void 0 } = $$props;
		function set5() {
			inputValue.set(5);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 5, 0);
		$$renderer.push(`read: ${$.escape($.store_get($$store_subs ??= {}, "$inputValue", inputValue))}</button>`);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", $.store_get($$store_subs ??= {}, "$inputValue", inputValue))}/>`);
		$.push_element($$renderer, "input", 6, 0);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { inputValue });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
