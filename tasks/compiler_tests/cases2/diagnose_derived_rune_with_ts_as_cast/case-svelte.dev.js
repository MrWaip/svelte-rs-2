App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let status = $.tag($.derived(() => $$props.error ? "error" : $$props.fallback), "status");
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(status)));
	$.append($$anchor, span);
	return $.pop($$exports);
}
