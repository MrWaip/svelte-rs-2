import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $a = () => $.store_get(a, "$a", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = writable(1);
	const obj = {};
	function run() {
		$.store_set(a, $.fallback(obj.$a, 5));
	}
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $a()));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
