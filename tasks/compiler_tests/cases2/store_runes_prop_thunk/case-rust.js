import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	const $store = () => $.store_get($$props.store, "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $store()));
	$.append($$anchor, p);
	$$cleanup();
}
