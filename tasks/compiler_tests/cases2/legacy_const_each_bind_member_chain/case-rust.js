import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	const obj = $.mutable_source({
		keys: ["a"],
		fields: { a: { value: 0 } }
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(obj).keys, (key) => key, ($$anchor, key) => {
		const field = $.derived_safe_equal(() => $.get(obj).fields[$.get(key)]);
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(field).value, ($$value) => $.get(field).value = $$value);
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
