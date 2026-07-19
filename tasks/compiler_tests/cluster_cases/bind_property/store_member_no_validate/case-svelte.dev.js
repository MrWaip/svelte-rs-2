App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<input type="number"/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $obj = () => ($.validate_store(obj, "obj"), $.store_get(obj, "$obj", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const obj = writable({ a: 1 });
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, function get() {
		return $obj().a;
	}, function set($$value) {
		$.store_mutate(obj, $.untrack($obj).a = $$value, $.untrack($obj));
	});
	$.append($$anchor, input);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
