import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	let items = $.proxy([{ done: false }]);
	let filtered = $.derived(() => items);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => $.get(filtered), $.index, ($$anchor, item, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_checked(input, () => $.get(item).done, ($$value) => $.get(item).done = $$value);
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
