import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<li> </li>`);
var root = $.from_html(`<ol></ol>`);
export default function App($$anchor) {
	const facts = [
		"Cats have five toes on their front paws, but only four on the back.",
		"A group of flamingos is called a 'flamboyance'.",
		"Bananas are berries, but strawberries aren't."
	];
	var ol = root();
	$.each(ol, 23, () => facts, (fact, i) => i, ($$anchor, fact) => {
		var li = root_1();
		var text = $.child(li, true);
		$.reset(li);
		$.template_effect(() => $.set_text(text, $.get(fact)));
		$.append($$anchor, li);
	});
	$.reset(ol);
	$.append($$anchor, ol);
}
