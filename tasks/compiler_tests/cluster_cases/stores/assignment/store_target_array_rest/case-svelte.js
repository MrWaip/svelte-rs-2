import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $a = () => $.store_get(a, "$a", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = writable(1);
	let rest = $.mutable_source();
	let arr = [
		1,
		2,
		3
	];
	function run() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.store_set(a, $$array[0]);
			$.set(rest, $$array.slice(1));
		})(arr);
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$a() ?? ""}${($.get(rest), $.untrack(() => $.get(rest).length)) ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
