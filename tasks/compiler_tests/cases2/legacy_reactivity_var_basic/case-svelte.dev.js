import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var count = $.tag($.mutable_source(0), "count");
	function increment() {
		$.set(count, $.safe_get(count) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `clicks: ${$.safe_get(count) ?? ""}`));
	$.delegated("click", button, increment);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
