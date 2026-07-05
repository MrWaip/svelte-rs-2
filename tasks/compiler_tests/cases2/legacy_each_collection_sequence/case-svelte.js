import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => [
		1,
		2,
		3
	]);
	function sideFx() {}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(items()), $.untrack(() => (sideFx(), items()))), $.index, ($$anchor, item) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
