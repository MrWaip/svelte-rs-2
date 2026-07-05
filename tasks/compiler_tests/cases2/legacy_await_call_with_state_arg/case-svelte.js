import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let id = $.prop($$props, "id", 8, 1);
	async function fetchData(arg) {
		return arg;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => ($.deep_read_state(id()), $.untrack(() => fetchData(id()))), null, ($$anchor, v) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
