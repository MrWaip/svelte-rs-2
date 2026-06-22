import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $c = () => $.store_get(c(), "$c", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let tmp = {
		a: 1,
		c: writable(2)
	}, a = $.prop($$props, "a", 28, () => tmp.a), c = $.prop($$props, "c", 24, () => tmp.c);
	function inc() {
		$.update_prop(a);
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$c() ?? ""}`));
	$.event("click", button, inc);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
