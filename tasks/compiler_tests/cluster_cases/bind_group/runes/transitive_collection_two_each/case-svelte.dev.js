App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let list = $.tag_proxy($.proxy([{ data: [{ value: [] }] }]), "list");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => list, $.index, ($$anchor, $$item, $$index_1) => {
		let data = () => $.get($$item).data;
		data();
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 17, data, $.index, ($$anchor, item, $$index) => {
			var input = root();
			$.remove_input_defaults(input);
			$.validate_binding("bind:group={item.value}", [], () => $.get(item), () => "value", 7, 25);
			input.value = input.__value = "a";
			$.bind_group(binding_group, [$$index, $$index_1], input, function get() {
				return $.get(item).value;
			}, function set($$value) {
				$.get(item).value = $$value;
			});
			$.append($$anchor, input);
		}), "each", App, 6, 1);
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
