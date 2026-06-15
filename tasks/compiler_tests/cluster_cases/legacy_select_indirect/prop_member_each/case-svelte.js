import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<option> </option>`);
var root_2 = $.from_html(`<p> </p>`);
var root = $.from_html(`<select></select> <label><input type="checkbox"/> </label> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let selected = $.prop($$props, "selected", 12);
	let tasks = $.prop($$props, "tasks", 8);
	$.init();
	var fragment = root();
	var select = $.first_child(fragment);
	$.each(select, 5, tasks, $.index, ($$anchor, task) => {
		var option = root_1();
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
	});
	$.reset(select);
	var label = $.sibling(select, 2);
	var input = $.child(label);
	$.remove_input_defaults(input);
	var text_1 = $.sibling(input);
	$.reset(label);
	var node = $.sibling(label, 2);
	$.each(node, 1, () => ($.deep_read_state(tasks()), $.untrack(() => tasks().filter((t) => !t.done))), $.index, ($$anchor, task) => {
		var p = root_2();
		var text_2 = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text_2, ($.get(task), $.untrack(() => $.get(task).description))));
		$.append($$anchor, p);
	});
	$.template_effect(() => $.set_text(text_1, ` ${($.deep_read_state(selected()), $.untrack(() => selected().description)) ?? ""}`));
	$.bind_select_value(select, selected);
	$.bind_checked(input, () => selected().done, ($$value) => (selected(selected().done = $$value, true), $.invalidate_inner_signals(() => {
		tasks();
	})));
	$.append($$anchor, fragment);
	$.pop();
}
