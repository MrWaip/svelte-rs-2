import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[6, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const store = $.tag($.mutable_source({
		items: ["a", "b"],
		data: {
			a: 1,
			b: 2
		}
	}), "store");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(store).items, $.index, ($$anchor, item) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return $.get(store).data[$.get(item)];
		}, function set($$value) {
			$.mutate(store, $.get(store).data[$.get(item)] = $$value);
		});
		$.append($$anchor, input);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
