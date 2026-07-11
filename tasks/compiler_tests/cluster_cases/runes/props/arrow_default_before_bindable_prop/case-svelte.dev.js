App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let format = $.prop($$props, "format", 3, (x) => x);
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(($0) => $.set_text(text, `${$0 ?? ""}${$$props.extra ?? ""}`), [() => format()($$props.value)]);
	$.append($$anchor, span);
	return $.pop($$exports);
}
