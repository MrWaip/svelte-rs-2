import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<input/> <button>swap</button>`, 1), App[$.FILENAME], [[8, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $s = () => ($.validate_store($.get(s), "s"), $.store_get($.get(s), "$s", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let s = $.mutable_source(writable(0));
	function swap() {
		$.store_unsub($.set(s, writable(1)), "$s", $$stores);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	$.bind_value(input, function get() {
		return $s();
	}, function set($$value) {
		$.store_set($.get(s), $$value);
	});
	$.delegated("click", button, swap);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
