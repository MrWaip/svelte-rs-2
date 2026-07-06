App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let metrics = writable([
			1,
			2,
			3
		]);
		let group = [];
		let total = $.derived(() => $.store_get($$store_subs ??= {}, "$metrics", metrics).length);
		$$renderer.push(`<input type="radio"${$.attr("checked", group === "a", true)} value="a"/>`);
		$.push_element($$renderer, "input", 8, 0);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", group === "b", true)} value="b"/>`);
		$.push_element($$renderer, "input", 9, 0);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`${$.escape(total())}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
