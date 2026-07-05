import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		let rest = () => $.exclude_from_object($.get($$item), ["a"]);
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${(rest(), $.untrack(() => rest().b)) ?? ""}`));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
