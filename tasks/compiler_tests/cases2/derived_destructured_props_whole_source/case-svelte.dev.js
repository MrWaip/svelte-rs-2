App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const props = $.rest_props($$props, rest_excludes, "props");
	const a = $.tag($.derived(() => $$props.a), "a"), b = $.tag($.derived(() => $$props.b), "b");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""},${$.get(b) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
