App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <button>inc</button>`, 1), App[$.FILENAME], [[6, 6]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	let b = $.tag($.state(0), "b");
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, function click() {
		$.update(a);
		$.update(b);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
