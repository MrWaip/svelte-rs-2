App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 29]]);
var select_content = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var root_1 = $.add_locations($.from_html(`<select><!></select>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root_1();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var node = $.first_child(fragment);
		$.add_svelte_meta(() => $.each(node, 17, () => $$props.items, $.index, ($$anchor, item) => {
			var div = root();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, div);
		}), "each", App, 5, 8);
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
