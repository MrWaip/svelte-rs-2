App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { promise } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		$.await($$renderer, $.store_get($$store_subs ??= {}, "$promise", promise), () => {
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 5, 17);
			$$renderer.push(`p</p>`);
			$.pop_element();
		}, (value) => {
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 5, 38);
			$$renderer.push(`${$.escape(value)}</p>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
