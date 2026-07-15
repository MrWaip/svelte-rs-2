App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const makeStore = $.tag($.derived(() => $$props.config.makeStore), "makeStore");
	const entries = $.tag($.derived(() => $.get(makeStore)()), "entries");
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(entries).x));
	$.append($$anchor, span);
	return $.pop($$exports);
}
