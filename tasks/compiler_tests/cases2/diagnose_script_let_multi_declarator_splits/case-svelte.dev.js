App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = 1;
	let b = 2;
	function bump() {
		a += 1;
		b += 1;
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}-${b ?? ""}`));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
