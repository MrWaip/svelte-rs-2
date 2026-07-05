import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item)));
		let a = () => $.get($$array)[0];
		let rest = () => $.get($$array).slice(1);
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${(rest(), $.untrack(() => rest()[0])) ?? ""}`));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
