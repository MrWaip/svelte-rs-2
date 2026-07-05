import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <strong> </strong> <!>`, 1), App[$.FILENAME], [[11, 0], [12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let title = $.tag($.mutable_source("x"), "title");
	let count = $.tag($.mutable_source(0), "count");
	function tick() {
		$.set(title, "y");
		$.set(count, 1);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var strong = $.sibling(p, 2);
	var text_1 = $.child(strong, true);
	$.reset(strong);
	var node = $.sibling(strong, 2);
	{
		$.validate_dynamic_element_tag(() => "div");
		$.validate_void_dynamic_element(() => "div");
		$.element(node, () => "div", false, ($$element, $$anchor) => {
			var text_2 = $.text();
			$.template_effect(() => $.set_text(text_2, `Dyn: ${$.get(title) ?? ""}`));
			$.append($$anchor, text_2);
		}, void 0, [13, 0]);
	}
	$.template_effect(() => {
		$.set_text(text, $.get(title));
		$.set_text(text_1, $.get(count));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
