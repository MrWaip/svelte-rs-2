import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<input/> <button>swap</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get($.get(s), "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let s = $.mutable_source(writable(0));
	function swap() {
		$.store_unsub($.set(s, writable(1)), "$s", $$stores);
	}
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	$.bind_value(input, $s, ($$value) => $.store_set($.get(s), $$value));
	$.delegated("click", button, swap);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
