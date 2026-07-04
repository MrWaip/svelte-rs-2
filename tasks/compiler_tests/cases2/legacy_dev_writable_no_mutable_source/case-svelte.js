import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $total = () => $.store_get(total, "$total", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let total = writable(0);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $total()));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
