App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let eq = $.tag($.state($.proxy($.strict_equals($$props.a, $$props.b))), "eq");
	function toggle() {
		$.set(eq, !$.get(eq));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(eq)));
	$.delegated("click", button, toggle);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
