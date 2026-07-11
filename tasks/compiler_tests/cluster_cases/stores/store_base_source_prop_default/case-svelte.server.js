import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { selectionWStore = undefined } = $$props;
		$$renderer.push(`<div>${$.escape($.store_get($$store_subs ??= {}, "$selectionWStore", selectionWStore)?.value)}</div>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
