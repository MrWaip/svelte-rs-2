import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<li> </li>`);
var root_1 = $.from_html(`<ul><!></ul>`);
export default function App($$anchor) {
	let deferred = $.proxy(Promise.withResolvers());
	var ul = root_1();
	var node = $.child(ul);
	$.async(node, [], [() => deferred.promise], (node, $$collection) => {
		$.each(node, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var li = root();
			var text = $.child(li, true);
			$.reset(li);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, li);
		});
	});
	$.reset(ul);
	$.append($$anchor, ul);
}
