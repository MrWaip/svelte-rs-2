App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function makeValue() {
		return 42;
	}
	const value = $.tag($.derived(makeValue), "value");
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.append($$anchor, span);
	return $.pop($$exports);
}
