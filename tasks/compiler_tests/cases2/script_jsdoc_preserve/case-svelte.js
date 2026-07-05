import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>toggle</button>`);
export default function App($$anchor) {
	/** @type {Function | undefined} */
	let show = $.state(void 0);
	/** Toggles the optional callback when clicked. */
	function toggle() {
		if ($.get(show)) $.get(show)();
	}
	var button = root();
	$.delegated("click", button, toggle);
	$.append($$anchor, button);
}
$.delegate(["click"]);
$.create_custom_element(App, {}, [], [], { mode: "open" });
