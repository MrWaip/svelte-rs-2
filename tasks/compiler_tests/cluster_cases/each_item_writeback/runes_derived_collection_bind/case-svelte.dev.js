App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{ done: false }]), "items");
	let filtered = $.tag($.derived(() => items), "filtered");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => $.get(filtered), $.index, ($$anchor, item, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.validate_binding("bind:checked={item.done}", [], () => $.get(item), () => "done", 7, 24);
		$.bind_checked(input, function get() {
			return $.get(item).done;
		}, function set($$value) {
			$.get(item).done = $$value;
		});
		$.append($$anchor, input);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
