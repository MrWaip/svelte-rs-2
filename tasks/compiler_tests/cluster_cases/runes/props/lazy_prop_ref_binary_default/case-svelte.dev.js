App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let b = $.prop($$props, "b", 19, () => $$props.a), c = $.prop($$props, "c", 19, () => b() * b());
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$$props.a ?? ""}${b() ?? ""}${c() ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
