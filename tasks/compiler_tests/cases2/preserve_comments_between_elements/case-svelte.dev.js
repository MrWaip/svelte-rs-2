App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>before</p> <!-- between --> <p> </p>`, 1), App[$.FILENAME], [[5, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	var p = $.sibling(node, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `after ${$$props.value ?? ""}`));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
