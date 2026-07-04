import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let cards = $.tag($.mutable_source([{ fav: false }]), "cards");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.get(cards), $.untrack(() => $.get(cards).filter((c) => !c.fav))), $.index, ($$anchor, card, $$index) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, ($.get(card), $.untrack(() => $.get(card).fav))));
		$.event("click", button, function click() {
			$.get(card).fav = !$.get(card).fav, $.invalidate_inner_signals(() => $.get(cards));
		});
		$.append($$anchor, button);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
