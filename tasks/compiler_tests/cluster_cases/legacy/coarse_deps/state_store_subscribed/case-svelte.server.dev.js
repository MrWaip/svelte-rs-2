App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let state = "hello";
		function update() {
			state = state + "!";
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`update</button>`);
		$.pop_element();
		$$renderer.push(` `);
		if (state) {
			$$renderer.push("<!--[0-->");
			const len = state.length;
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 13, 4);
			$$renderer.push(`${$.escape(len)} / ${$.escape($.store_get($$store_subs ??= {}, "$state", state))}</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
