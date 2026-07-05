import * as $ from "svelte/internal/client";
var root = $.from_html(`<li> </li>`);
var root_1 = $.from_html(`<ol></ol>`);
export default function App($$anchor) {
	const facts = [
		"Cats have five toes on their front paws, but only four on the back.",
		"A group of flamingos is called a 'flamboyance'.",
		"Bananas are berries, but strawberries aren't."
	];
	var ol = root_1();
	$.each(ol, 21, () => facts, $.index, ($$anchor, fact) => {
		var li = root();
		var text = $.child(li, true);
		$.reset(li);
		$.template_effect(() => $.set_text(text, $.get(fact)));
		$.append($$anchor, li);
	});
	$.reset(ol);
	$.append($$anchor, ol);
}
