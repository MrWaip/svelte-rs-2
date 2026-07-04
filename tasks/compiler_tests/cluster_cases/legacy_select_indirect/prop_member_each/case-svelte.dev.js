import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[9, 2]]);
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[18, 1]]);
var root_2 = $.add_locations($.from_html(`<select></select> <label><input type="checkbox"/> </label> <!>`, 1), App[$.FILENAME], [[7, 0], [
	13,
	0,
	[[14, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let selected = $.prop($$props, "selected", 12);
	let tasks = $.prop($$props, "tasks", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root_2();
	var select = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(select, 5, tasks, $.index, ($$anchor, task) => {
		var option = root();
		var text = $.child(option, true);
		$.reset(option);
		var option_value = {};
		$.template_effect(() => {
			$.set_text(text, ($.get(task), $.untrack(() => $.get(task).description)));
			if (option_value !== (option_value = $.get(task))) {
				option.value = (option.__value = $.get(task)) ?? "";
			}
		});
		$.append($$anchor, option);
	}), "each", App, 8, 1);
	$.reset(select);
	var label = $.sibling(select, 2);
	var input = $.child(label);
	$.remove_input_defaults(input);
	var text_1 = $.sibling(input);
	$.reset(label);
	var node = $.sibling(label, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.deep_read_state(tasks()), $.untrack(() => tasks().filter((t) => !t.done))), $.index, ($$anchor, task) => {
		var p = root_1();
		var text_2 = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text_2, ($.get(task), $.untrack(() => $.get(task).description))));
		$.append($$anchor, p);
	}), "each", App, 17, 0);
	$.template_effect(() => $.set_text(text_1, ` ${($.deep_read_state(selected()), $.untrack(() => selected().description)) ?? ""}`));
	$.bind_select_value(select, function get() {
		return selected();
	}, function set($$value) {
		selected($$value);
	});
	$.bind_checked(input, function get() {
		return selected().done;
	}, function set($$value) {
		$$ownership_validator.mutation(null, ["selected", "done"], (selected(selected().done = $$value, true), $.invalidate_inner_signals(() => {
			tasks();
		})), 14, 38);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
