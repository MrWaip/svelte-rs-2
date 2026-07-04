App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
let shared = $.tag($.state(0), "shared");
let doubled = $.tag($.derived(() => $.get(shared) * 2), "doubled");
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function increment() {
		$.update(shared);
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
