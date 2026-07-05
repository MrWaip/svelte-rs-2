import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let list = [{
		id: "x",
		data: [{
			id: 1,
			data: []
		}]
	}];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => list, $.index, ($$anchor, $$item, $$index_1) => {
		let id = () => $.get($$item).id;
		id();
		let data = () => $.get($$item).data;
		data();
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 1, data, $.index, ($$anchor, item, $$index) => {
			var input = root();
			$.remove_input_defaults(input);
			input.value = input.__value = "a";
			$.template_effect(() => $.set_attribute(input, "data-index", `${id() ?? ""}-${($.get(item), $.untrack(() => $.get(item).id)) ?? ""}`));
			$.bind_group(binding_group, [$$index, $$index_1], input, function get() {
				return $.get(item).data;
			}, function set($$value) {
				$.get(item).data = $$value, $.invalidate_inner_signals(() => (data(), list));
			});
			$.append($$anchor, input);
		}), "each", App, 7, 1);
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
