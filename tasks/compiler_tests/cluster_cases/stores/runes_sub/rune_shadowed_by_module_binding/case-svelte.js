import * as $ from "svelte/internal/client";
const data = 0;
const derived = $.derived(() => data + 1);
const state = {
	data,
	derived: $.get(derived)
};
export default function App($$anchor) {
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	;
	;
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, state.data));
	$.append($$anchor, text);
	$$cleanup();
}
