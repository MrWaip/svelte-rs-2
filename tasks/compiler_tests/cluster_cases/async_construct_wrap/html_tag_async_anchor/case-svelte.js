import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1><!></h1>`);
export default function App($$anchor) {
	let deferred = $.proxy(Promise.withResolvers());
	var h1 = root();
	var node = $.child(h1);
	$.async(node, [], [() => deferred.promise], (node, $$html) => {
		$.html(node, () => $.get($$html));
	});
	$.reset(h1);
	$.append($$anchor, h1);
}
