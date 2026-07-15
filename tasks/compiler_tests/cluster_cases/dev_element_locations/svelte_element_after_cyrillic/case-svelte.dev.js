App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>Текст <!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.sibling($.child(div));
	{
		$.validate_dynamic_element_tag(() => $$props.tag);
		$.validate_void_dynamic_element(() => $$props.tag);
		$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
			var text = $.text("жирный");
			$.append($$anchor, text);
		}, void 0, [5, 11]);
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
