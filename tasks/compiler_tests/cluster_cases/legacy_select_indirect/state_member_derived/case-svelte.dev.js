import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>Select</option><option>US</option></select>`), App[$.FILENAME], [[
	15,
	0,
	[[19, 1], [20, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const data = $.mutable_source();
	const details = $.mutable_source();
	const default_details = { country: "" };
	$.legacy_pre_effect(() => {}, () => {
		$.set(data, {
			locked: false,
			details: null
		});
	});
	$.legacy_pre_effect(() => $.get(data), () => {
		$.set(details, $.get(data).details ?? default_details);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "us";
	$.reset(select);
	$.template_effect(() => select.disabled = ($.get(data), $.untrack(() => $.get(data).locked)));
	$.bind_select_value(select, function get() {
		return $.get(details).country;
	}, function set($$value) {
		$.mutate(details, $.get(details).country = $$value), $.invalidate_inner_signals(() => {
			$.get(data);
		});
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
