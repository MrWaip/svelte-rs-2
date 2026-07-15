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
	const obj = {
		$a: 1,
		c: 2,
		d: 3
	};
	function run() {
		$.store_set(a, obj.$a), $.set(rest, $.exclude_from_object(obj, ["$a"]));
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$a() ?? ""}${($.get(rest), $.untrack(() => $.get(rest).c)) ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
