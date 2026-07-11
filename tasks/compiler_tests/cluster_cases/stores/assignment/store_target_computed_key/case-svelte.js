import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $x = () => $.store_get(x, "$x", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = writable(1);
	const k = "a";
	const obj = { a: 42 };
	function run() {
		$.store_set(x, obj[k]);
	}
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $x()));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
