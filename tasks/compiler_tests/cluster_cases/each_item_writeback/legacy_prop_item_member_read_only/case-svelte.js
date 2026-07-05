import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let arr = $.prop($$props, "arr", 24, () => [{ prop: "foo" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, arr, $.index, ($$anchor, o) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, ($.get(o), $.untrack(() => $.get(o).prop))));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
