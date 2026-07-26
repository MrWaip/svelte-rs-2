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
	{
		let $0 = $.derived(() => delay($.get(x)));
		$.slot(node, $$props, "default", { get value() {
			return $.get($0);
		} }, null);
	}
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
