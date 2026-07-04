App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.from_html(`<div> </div>`, 1), App[$.FILENAME], [[5, 8]]);
var root = $.add_locations($.from_html(`<select><!></select>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var div = $.first_child(fragment);
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $$props.label));
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
