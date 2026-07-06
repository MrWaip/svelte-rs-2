App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let meta = $.fallback($$props["meta"], () => writable({ hint: "x" }), true);
		let component = Inner;
		if (component) {
			$$renderer.push("<!--[-->");
			component($$renderer, { $$slots: { caption: ($$renderer) => {
				$$renderer.push(`<span slot="caption">`);
				$.push_element($$renderer, "span", 11, 4);
				$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$meta", meta).hint || "")}</span>`);
				$.pop_element();
			} } });
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { meta });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
