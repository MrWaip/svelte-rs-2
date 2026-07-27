import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div> <button> </button>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let count = $.tag($.mutable_source(0), "count");
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "title", [() => $$sanitized_props.x]);
	var text = $.child(div, true);
	$.reset(div);
	var button = $.sibling(div, 2);
	var text_1 = $.child(button, true);
	$.reset(button);
	$.template_effect(() => {
		$.set_text(text, a());
		$.set_text(text_1, $.get(count));
	});
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
