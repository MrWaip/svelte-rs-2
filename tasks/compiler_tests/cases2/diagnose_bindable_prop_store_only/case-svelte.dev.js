App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button> <input/>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $inputValue = () => ($.validate_store($$props.inputValue, "inputValue"), $.store_get($$props.inputValue, "$inputValue", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	function set5() {
		$$props.inputValue.set(5);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button);
	$.reset(button);
	var input = $.sibling(button, 2);
	$.remove_input_defaults(input);
	$.template_effect(() => $.set_text(text, `read: ${$inputValue() ?? ""}`));
	$.delegated("click", button, set5);
	$.bind_value(input, function get() {
		return $inputValue();
	}, function set($$value) {
		$.store_set($$props.inputValue, $$value);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
