App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<br/> <button>inc</button>`, 1), App[$.FILENAME], [[5, 0], [5, 8]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var text = $.sibling($.first_child(fragment), 1, true);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.delegated("click", button, function click() {
		return $.update(a);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
