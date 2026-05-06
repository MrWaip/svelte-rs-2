import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var count = writable(0);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $count()));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
