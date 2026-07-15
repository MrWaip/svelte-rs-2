import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $a = () => $.store_get(a, "$a", $$stores);
	const $b = () => $.store_get(b, "$b", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = writable(1);
	const b = writable(2);
	const obj = {
		$a: 10,
		$b: 20
	};
	function run() {
		$.store_set(a, obj.$a), $.store_set(b, obj.$b);
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$a() ?? ""}${$b() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
