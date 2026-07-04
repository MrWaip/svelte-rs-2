import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="description" content="A"/>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<span>x</span> `, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.append($$anchor, meta);
	});
	var text = $.sibling($.first_child(fragment));
	$.template_effect(() => $.set_text(text, ` ${foo() ?? ""}`));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
