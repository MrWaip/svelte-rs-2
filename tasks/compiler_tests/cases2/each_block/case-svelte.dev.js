App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <div> </div>`, 1), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.prop($$props, "items", 19, () => []);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, items, $.index, ($$anchor, item) => {
		$.next();
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div);
		$.reset(div);
		$.template_effect(() => {
			$.set_text(text, `${$.get(item) ?? ""} `);
			$.set_text(text_1, `${$.get(item) ?? ""} + example`);
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
