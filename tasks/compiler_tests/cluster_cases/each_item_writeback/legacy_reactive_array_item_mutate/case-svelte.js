import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let cards = $.mutable_source([{ fav: false }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(cards), $.index, ($$anchor, card, $$index) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, ($.get(card), $.untrack(() => $.get(card).fav))));
		$.event("click", button, () => {
			$.get(card).fav = !$.get(card).fav, $.invalidate_inner_signals(() => $.get(cards));
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
