import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="radio"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let active = $.prop($$props, "active", 12);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => [
		1,
		2,
		3
	], $.index, ($$anchor, _, index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		input.value = input.__value = index;
		$.bind_group(binding_group, [], input, () => {
			index;
			return active();
		}, active);
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
