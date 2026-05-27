import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => []);
	let prefix = $.prop($$props, "prefix", 8, "");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 3, items, (item, i) => prefix() + i, ($$anchor, item) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
