App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fmt(x) {
		return x;
	}
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(($0) => $.set_text(text, `${$0 ?? ""} • ${$$props.card.tail ?? ""}`), [() => $$props.card.flag ? "A" : "B" + fmt($$props.card.x)]);
	$.append($$anchor, span);
	return $.pop($$exports);
}
