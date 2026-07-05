App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"id"
]);
var root = $.add_locations($.from_html(`<p> </p> <span> </span> <div> </div>`, 1), App[$.FILENAME], [
	[7, 0],
	[8, 0],
	[9, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let props = $.rest_props($$props, rest_excludes, "props");
	const label = $.tag($.derived(() => $$props.label + "!"), "label");
	const style = $.tag($.derived(() => $$props.style), "style");
	const color = $.tag($.derived(() => $$props.style.color), "color");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var span = $.sibling(p, 2);
	var text_1 = $.child(span, true);
	$.reset(span);
	var div = $.sibling(span, 2);
	var text_2 = $.child(div, true);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text, $.get(label));
		$.set_text(text_1, $$props.title);
		$.set_text(text_2, $$props.nested.deep.value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
