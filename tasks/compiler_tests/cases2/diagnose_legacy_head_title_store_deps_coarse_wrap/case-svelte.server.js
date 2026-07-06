import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const toolbar = writable({ title: "" });
		function getTitle(t) {
			return t;
		}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>${$.escape(getTitle($.store_get($$store_subs ??= {}, "$toolbar", toolbar).title))}</title>`);
			});
		});
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
