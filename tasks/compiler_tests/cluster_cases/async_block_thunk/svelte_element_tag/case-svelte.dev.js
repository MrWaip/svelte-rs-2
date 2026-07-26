import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [async () => (await $.save(delay($.get(x))))() + "div"], (node, $$tag) => {
		$.validate_dynamic_element_tag(() => $.get($$tag));
		$.validate_void_dynamic_element(() => $.get($$tag));
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			var text = $.text("content");
			$.append($$anchor, text);
		}, void 0, [11, 0]);
	});
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
