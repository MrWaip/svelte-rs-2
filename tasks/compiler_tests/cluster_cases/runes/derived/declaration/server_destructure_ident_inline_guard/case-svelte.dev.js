App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const phone = $.tag($.derived(() => $$props.source.phone), "phone"), rate = $.tag($.derived(() => $$props.source.rate), "rate");
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(() => $.set_text(text, `${$.get(phone) ?? ""}${$.get(rate) ?? ""}`));
	$.append($$anchor, span);
	return $.pop($$exports);
}
