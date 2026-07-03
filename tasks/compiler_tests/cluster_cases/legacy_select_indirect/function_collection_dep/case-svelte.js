import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
var root_1 = $.from_html(`<select></select> `, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let letters = $.prop($$props, "letters", 24, () => [
		"a",
		"b",
		"c"
	]);
	let selected = $.prop($$props, "selected", 28, () => ({ letter: "" }));
	function uppercase() {
		return letters().map((x) => x.toUpperCase());
	}
	$.init();
	var fragment = root_1();
	var select = $.first_child(fragment);
	$.each(select, 5, () => $.untrack(uppercase), $.index, ($$anchor, letter) => {
		var option = root();
		var text = $.child(option, true);
		$.reset(option);
		var option_value = {};
		$.template_effect(() => {
			$.set_text(text, $.get(letter));
			if (option_value !== (option_value = $.get(letter))) {
				option.value = (option.__value = $.get(letter)) ?? "";
			}
		});
		$.append($$anchor, option);
	});
	$.reset(select);
	var text_1 = $.sibling(select);
	$.template_effect(() => $.set_text(text_1, ` ${($.deep_read_state(selected()), $.untrack(() => selected().letter)) ?? ""}`));
	$.bind_select_value(select, () => selected().letter, ($$value) => (selected(selected().letter = $$value, true), $.invalidate_inner_signals(() => {
		uppercase;
	})));
	$.append($$anchor, fragment);
	$.pop();
}
