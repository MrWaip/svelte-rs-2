import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>load</button>`, 1);
export default function App($$anchor) {
	let count = $.mutable_source(0);
	async function load() {
		$.set(count, await Promise.resolve(1));
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, load);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
