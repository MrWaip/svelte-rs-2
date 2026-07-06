import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let meta = $.fallback($$props["meta"], () => writable({ disabled: false }), true);
		let x;
		let component = Inner;
		if (component) {
			$$renderer.push("<!--[-->");
			component($$renderer, { $$slots: { icon: ($$renderer) => {
				$$renderer.push(`<div slot="icon">`);
				if (x && !$.store_get($$store_subs ??= {}, "$meta", meta).disabled) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<div>hi</div>`);
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]--></div>`);
			} } });
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { meta });
	});
}
