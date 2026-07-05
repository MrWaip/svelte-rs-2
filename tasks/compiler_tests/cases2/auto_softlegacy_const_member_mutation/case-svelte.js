import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	const store = $.mutable_source({
		items: ["a", "b"],
		data: {
			a: 1,
			b: 2
		}
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(store).items, $.index, ($$anchor, item) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(store).data[$.get(item)], ($$value) => $.mutate(store, $.get(store).data[$.get(item)] = $$value));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
