import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let items = [{ id: "a" }, { id: "b" }];
	let selected = $.tag($.mutable_source(), "selected");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => items, $.index, ($$anchor, item) => {
		var input = root();
		$.remove_input_defaults(input);
		var input_value;
		$.template_effect(() => {
			if (input_value !== (input_value = ($.get(item), $.untrack(() => $.get(item).id)))) {
				input.value = (input.__value = ($.get(item), $.untrack(() => $.get(item).id))) ?? "";
			}
		});
		$.bind_group(binding_group, [], input, () => {
			$.get(item), $.untrack(() => $.get(item).id);
			return $.get(selected);
		}, function set($$value) {
			$.set(selected, $$value);
		});
		$.append($$anchor, input);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
