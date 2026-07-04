App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	let doubled = $.tag($.derived(() => $.get(count) * 2), "doubled");
	function increment() {
		$.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.delegated("click", button, increment);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
