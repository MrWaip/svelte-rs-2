import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = [{
		a: 1,
		b: 2,
		c: 3
	}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		let rest = () => $.exclude_from_object($.get($$item), ["a"]);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a() ?? ""}${(rest(), $.untrack(() => rest().b)) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
