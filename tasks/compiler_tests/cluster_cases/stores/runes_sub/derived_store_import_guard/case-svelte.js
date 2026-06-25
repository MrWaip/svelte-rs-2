import * as $ from "svelte/internal/client";
import { writable, derived } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $store = () => $.store_get(store, "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const store = writable(0);
	let doubled = $.derived(() => $store() * 2);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(doubled) ?? ""} ${$store() ?? ""}`));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
