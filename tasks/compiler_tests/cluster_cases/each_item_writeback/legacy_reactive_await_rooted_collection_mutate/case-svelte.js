import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve([{ fav: false }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, cards) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 1, () => ($.deep_read_state($.get(cards)), $.untrack(() => $.get(cards).filter((c) => !c.fav))), $.index, ($$anchor, card, $$index) => {
			var button = root_2();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, ($.get(card), $.untrack(() => $.get(card).fav))));
			$.event("click", button, () => {
				$.get(card).fav = !$.get(card).fav, $.invalidate_inner_signals(() => $.get(cards));
			});
			$.append($$anchor, button);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
