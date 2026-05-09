import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>bump</button> <div></div>`, 1), App[$.FILENAME], [[13, 0], [14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let title = $.mutable_source("t");
	let counter = $.mutable_source(0);
	let flag = $.mutable_source("x");
	function bump() {
		$.set(title, $.get(title) + "!");
		$.set(counter, $.get(counter) + 1);
		$.set(flag, $.get(flag) + "!");
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	$.template_effect(() => {
		$.set_attribute(div, "title", $.get(title));
		$.set_attribute(div, "data-counter", $.get(counter));
		$.set_attribute(div, "data-flag", $.get(flag));
	});
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
