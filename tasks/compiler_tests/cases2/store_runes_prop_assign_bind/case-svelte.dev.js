App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <button>reset</button>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $limitAmount = () => ($.validate_store($$props.limitAmount, "limitAmount"), $.store_get($$props.limitAmount, "$limitAmount", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	$.bind_value(input, function get() {
		return $limitAmount();
	}, function set($$value) {
		$.store_set($$props.limitAmount, $$value);
	});
	$.delegated("click", button, function click() {
		return $.store_set($$props.limitAmount, undefined);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
