import * as $ from "svelte/internal/server";
const data = 0;
const derived = $.derived(() => data + 1);
const state = {
	data,
	derived: derived()
};
export default function App($$renderer) {
	var $$store_subs;
	;
	;
	$$renderer.push(`<!---->${$.escape(state.data)}`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
