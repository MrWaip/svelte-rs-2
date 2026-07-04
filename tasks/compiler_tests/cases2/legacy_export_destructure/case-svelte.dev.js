import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		x: "a",
		z: ["b"]
	}, $$array = $.derived(() => $.to_array(tmp.z, 1)), foo = $.prop($$props, "foo", 24, () => $.fallback(tmp.x, "default-x")), bar = $.prop($$props, "bar", 24, () => $.get($$array)[0]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo() ?? ""}${bar() ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
