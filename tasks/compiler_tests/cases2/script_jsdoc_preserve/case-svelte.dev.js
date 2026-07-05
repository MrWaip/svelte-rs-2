App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>toggle</button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	/** @type {Function | undefined} */
	let show = $.tag($.state(void 0), "show");
	/** Toggles the optional callback when clicked. */
	function toggle() {
		if ($.get(show)) $.get(show)();
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, toggle);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
$.create_custom_element(App, {}, [], [], { mode: "open" });
