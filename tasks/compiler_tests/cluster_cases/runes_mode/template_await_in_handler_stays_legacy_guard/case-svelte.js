import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>inc</button> <button>double</button>`, 1);
export default function App($$anchor) {
	let count = $.mutable_source(0);
	async function compute(v) {
		return v * 2;
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	var button_1 = $.sibling(button, 2);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.delegated("click", button_1, async () => {
		$.set(count, await compute($.get(count)));
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
