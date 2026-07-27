App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $array = () => ($.validate_store(array, "array"), $.store_get(array, "$array", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const array = writable([{ name: "" }]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, $array, $.index, ($$anchor, item, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.validate_binding("bind:value={item.name}", [], () => ($.mark_store_binding(), $.get(item)), () => "name", 8, 8);
		$.bind_value(input, function get() {
			return $.get(item).name;
		}, function set($$value) {
			$.get(item).name = $$value, $.invalidate_store($$stores, "$array");
		});
		$.append($$anchor, input);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
