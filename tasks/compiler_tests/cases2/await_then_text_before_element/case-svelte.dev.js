App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <div> </div>`, 1), App[$.FILENAME], [[3, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => promise, null, ($$anchor, result) => {
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div, true);
		$.reset(div);
		$.template_effect(() => {
			$.set_text(text, `text ${$.get(result).name ?? ""} `);
			$.set_text(text_1, $.get(result).value);
		});
		$.append($$anchor, fragment_1);
	}), "await", App, 1, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
