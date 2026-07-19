App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	let tmp = $$props.o, a = $.tag_proxy($.proxy(tmp.a), "a"), b = $.tag_proxy($.proxy(tmp.b), "b");
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a ?? ""} ${b ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
