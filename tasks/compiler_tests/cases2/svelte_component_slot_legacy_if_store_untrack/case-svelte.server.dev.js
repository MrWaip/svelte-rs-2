App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let meta = $.fallback($$props["meta"], () => writable({ disabled: false }), true);
		let x;
		let component = Inner;
		if (component) {
			$$renderer.push("<!--[-->");
			component($$renderer, { $$slots: { icon: ($$renderer) => {
				$$renderer.push(`<div slot="icon">`);
				$.push_element($$renderer, "div", 12, 4);
				if (x && !$.store_get($$store_subs ??= {}, "$meta", meta).disabled) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<div>`);
					$.push_element($$renderer, "div", 14, 12);
					$$renderer.push(`hi</div>`);
					$.pop_element();
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]--></div>`);
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
