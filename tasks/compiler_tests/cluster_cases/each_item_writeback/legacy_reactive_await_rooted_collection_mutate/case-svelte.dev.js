import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let p = Promise.resolve([{ fav: false }]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => p, null, ($$anchor, cards) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 1, () => ($.deep_read_state($.get(cards)), $.untrack(() => $.get(cards).filter((c) => !c.fav))), $.index, ($$anchor, card, $$index) => {
			var button = root();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, ($.get(card), $.untrack(() => $.get(card).fav))));
			$.event("click", button, function click() {
				$.get(card).fav = !$.get(card).fav, $.invalidate_inner_signals(() => $.get(cards));
			});
			$.append($$anchor, button);
		}), "each", App, 7, 1);
		$.append($$anchor, fragment_1);
	}), "await", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
