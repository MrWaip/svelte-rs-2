import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get($.get(count), "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.mutable_source(writable(0));
	function swap() {
		$.store_unsub($.set(count, writable(10)), "$count", $$stores);
	}
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $count()));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
