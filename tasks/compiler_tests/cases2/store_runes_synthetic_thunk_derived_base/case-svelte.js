import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $store = () => $.store_get($.get(store), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.derived(() => $$props.manager.store);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $store()));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
