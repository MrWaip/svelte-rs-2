import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let active = $.prop($$props, "active", 12);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 0, () => [
		1,
		2,
		3
	], $.index, ($$anchor, _, index) => {
		var input = root();
		$.remove_input_defaults(input);
		input.value = input.__value = index;
		$.bind_group(binding_group, [], input, () => {
			index;
			return active();
		}, function set($$value) {
			active($$value);
		});
		$.append($$anchor, input);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
