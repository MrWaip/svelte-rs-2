App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<li> </li>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	var li = root();
	var text = $.child(li);
	$.reset(li);
	$.template_effect(($0, $1) => $.set_text(text, `${$0 ?? ""}${$1 ?? ""}`), [() => fn(), () => null && fn()]);
	$.append($$anchor, li);
	return $.pop($$exports);
}
