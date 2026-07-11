App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => $$props.keys, (key) => key, ($$anchor, key) => {
		const column = $.tag($.derived(() => $$props.columns[key]), "column");
		$.get(column);
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(column)));
		$.append($$anchor, div);
	}), "each", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
