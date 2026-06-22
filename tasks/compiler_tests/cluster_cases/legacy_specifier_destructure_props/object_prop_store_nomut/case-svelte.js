import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $c = () => $.store_get(c(), "$c", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let tmp = {
		a: 1,
		c: writable(2)
	}, a = $.prop($$props, "a", 24, () => tmp.a), c = $.prop($$props, "c", 24, () => tmp.c);
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$c() ?? ""}`));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
