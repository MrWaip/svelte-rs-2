import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<p>empty</p>`);
var root_2 = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let n = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root_2();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [() => delay([$.get(n)])], (node, $$collection) => {
		$.each(node, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p);
		}, ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		});
	});
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
