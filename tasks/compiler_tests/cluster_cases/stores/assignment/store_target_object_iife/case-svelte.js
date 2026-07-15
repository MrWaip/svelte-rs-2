import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $x = () => $.store_get(x, "$x", $$stores);
	const $y = () => $.store_get(y, "$y", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = writable(1);
	const y = writable(2);
	function run() {
		(($$value) => {
			$.store_set(x, $$value.a);
			$.store_set(y, $$value.b);
		})({
			a: 9,
			b: 10
		});
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$x() ?? ""}${$y() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
