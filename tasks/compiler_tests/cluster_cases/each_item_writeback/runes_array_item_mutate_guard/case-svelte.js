import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let cards = $.proxy([{ fav: false }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => cards, $.index, ($$anchor, card, $$index) => {
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(card).fav));
		$.delegated("click", button, () => {
			$.get(card).fav = !$.get(card).fav;
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
