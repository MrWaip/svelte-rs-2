App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <div> </div>`, 1), App[$.FILENAME], [[11, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => items, (item) => item, ($$anchor, item) => {
		$.next();
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div, true);
		$.reset(div);
		$.template_effect(($0) => {
			$.set_text(text, `${$0 ?? ""} `);
			$.set_text(text_1, item);
		}, [() => (() => {
			$.user_effect(() => {
				items;
			});
		})()]);
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
