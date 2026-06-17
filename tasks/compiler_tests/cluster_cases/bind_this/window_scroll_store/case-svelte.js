import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $y = () => $.store_get(y, "$y", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const y = writable(0);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $y()));
	$.bind_window_scroll("y", $y, ($$value) => $.store_set(y, $$value));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
