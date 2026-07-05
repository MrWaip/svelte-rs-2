App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { f, g } from "./x";
var root = $.add_locations($.from_html(`<a> </a>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let url = $.prop($$props, "url", 3, ""), label = $.prop($$props, "label", 3, "");
	var $$exports = { ...$.legacy_api() };
	var a = root();
	var text = $.child(a, true);
	$.reset(a);
	$.template_effect(($0, $1) => {
		$.set_attribute(a, "href", $0);
		$.set_text(text, $1);
	}, [() => f(url()), () => g(label())]);
	$.append($$anchor, a);
	return $.pop($$exports);
}
