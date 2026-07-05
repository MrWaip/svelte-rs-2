App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 3]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let ops = $.tag_proxy($.proxy([{
		args: [{
			value: [],
			options: [{ value: "a" }]
		}],
		id: 1
	}]), "ops");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => ops, ({ args, id }) => id, ($$anchor, $$item, $$index_2) => {
		let args = () => $.get($$item).args;
		args();
		let id = () => $.get($$item).id;
		id();
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 17, args, $.index, ($$anchor, arg, $$index_1) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.add_svelte_meta(() => $.each(node_2, 17, () => $.get(arg).options, $.index, ($$anchor, $$item) => {
				let value = () => $.get($$item).value;
				value();
				var input = root();
				$.remove_input_defaults(input);
				$.validate_binding("bind:group={arg.value}", [], () => $.get(arg), () => "value", 8, 26);
				var input_value;
				$.template_effect(() => {
					if (input_value !== (input_value = value())) {
						input.value = (input.__value = value()) ?? "";
					}
				});
				$.bind_group(binding_group, [$$index_1, $$index_2], input, () => {
					value();
					return $.get(arg).value;
				}, function set($$value) {
					$.get(arg).value = $$value;
				});
				$.append($$anchor, input);
			}), "each", App, 7, 2);
			$.append($$anchor, fragment_2);
		}), "each", App, 6, 1);
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
