import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<button>add</button> <!>`, 1);
export default function App($$anchor) {
	let rows = $.mutable_source([{ name: "a" }, { name: "b" }]);
	function add() {
		$.set(rows, [...$.get(rows), { name: "c" }]);
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 1, () => $.get(rows), $.index, ($$anchor, row, idx) => {
		const label = $.derived_safe_equal(() => ($.get(row), idx, $.untrack(() => $.get(row).name + idx)));
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(label)));
		$.append($$anchor, p);
	});
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
