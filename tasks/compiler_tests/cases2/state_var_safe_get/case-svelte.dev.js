App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var count = $.tag($.state(0), "count");
	var name = "hello";
	$.set(count, $.safe_get(count) + 1);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.safe_get(count) ?? ""} hello`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
