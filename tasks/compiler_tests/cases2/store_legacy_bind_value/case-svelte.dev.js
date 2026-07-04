import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { name } from "./stores";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $name = () => ($.validate_store(name, "name"), $.store_get(name, "$name", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, function get() {
		return $name();
	}, function set($$value) {
		$.store_set(name, $$value);
	});
	$.append($$anchor, input);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
