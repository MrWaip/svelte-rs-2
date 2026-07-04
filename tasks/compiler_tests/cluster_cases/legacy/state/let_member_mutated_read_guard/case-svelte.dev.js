import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.tag($.mutable_source({ k: 1 }), "obj");
	const bump = () => {
		$.mutate(obj, $.get(obj).k = 2);
	};
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(obj), $.untrack(() => $.get(obj).k))));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
