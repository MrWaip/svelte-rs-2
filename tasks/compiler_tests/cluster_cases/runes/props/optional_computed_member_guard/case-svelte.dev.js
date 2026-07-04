App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const props = $.rest_props($$props, rest_excludes, "props");
	let title = $.tag($.derived(() => props?.["title"]), "title");
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(title)));
	$.append($$anchor, text);
	return $.pop($$exports);
}
