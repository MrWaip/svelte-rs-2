App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	let rest = $.tag($.state(0), "rest");
	let obj = {
		a: 1,
		x: 2
	};
	function update() {
		$.set(a, obj.a, true), $.set(rest, $.exclude_from_object(obj, ["a"]), true);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(rest) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
