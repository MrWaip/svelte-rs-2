import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[13, 2]]);
var root_1 = $.add_locations($.from_html(`<select></select> `, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let letters = $.prop($$props, "letters", 24, () => [
		"a",
		"b",
		"c"
	]);
	let selected = $.prop($$props, "selected", 28, () => ({ letter: "" }));
	function uppercase() {
		return letters().map((x) => x.toUpperCase());
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root_1();
	var select = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(select, 5, () => $.untrack(uppercase), $.index, ($$anchor, letter) => {
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
	}), "each", App, 12, 1);
	$.reset(select);
	var text_1 = $.sibling(select);
	$.template_effect(() => $.set_text(text_1, ` ${($.deep_read_state(selected()), $.untrack(() => selected().letter)) ?? ""}`));
	$.bind_select_value(select, function get() {
		return selected().letter;
	}, function set($$value) {
		$$ownership_validator.mutation(null, ["selected", "letter"], (selected(selected().letter = $$value, true), $.invalidate_inner_signals(() => {
			uppercase;
		})), 11, 20);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
