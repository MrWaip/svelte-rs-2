import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	let refs = $.mutable_source([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item, idx) => {
		var input = root();
		$.bind_this(input, ($$value, idx) => $.mutate(refs, $.get(refs)[idx] = $$value), (idx) => $.get(refs)?.[idx], () => [idx]);
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
