App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<li> </li>`), App[$.FILENAME], [[11, 2]]);
var root_1 = $.add_locations($.from_html(`<ol></ol>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const facts = [
		"Cats have five toes on their front paws, but only four on the back.",
		"A group of flamingos is called a 'flamboyance'.",
		"Bananas are berries, but strawberries aren't."
	];
	var $$exports = { ...$.legacy_api() };
	var ol = root_1();
	$.add_svelte_meta(() => $.each(ol, 21, () => facts, $.index, ($$anchor, fact) => {
		var li = root();
		var text = $.child(li, true);
		$.reset(li);
		$.template_effect(() => $.set_text(text, $.get(fact)));
		$.append($$anchor, li);
	}), "each", App, 10, 1);
	$.reset(ol);
	$.append($$anchor, ol);
	return $.pop($$exports);
}
