import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let meta = $.fallback($$props["meta"], () => writable({ hint: "x" }), true);
		let component = Inner;
		if (component) {
			$$renderer.push("<!--[-->");
			component($$renderer, { $$slots: { caption: ($$renderer) => {
				$$renderer.push(`<span slot="caption">${$.escape($.store_get($$store_subs ??= {}, "$meta", meta).hint || "")}</span>`);
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
