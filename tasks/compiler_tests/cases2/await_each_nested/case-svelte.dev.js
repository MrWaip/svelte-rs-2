App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<li> </li>`), App[$.FILENAME], [[4, 3]]);
var root_1 = $.add_locations($.from_html(`<ul></ul>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => items, null, ($$anchor, result) => {
		var ul = root_1();
		$.add_svelte_meta(() => $.each(ul, 21, () => $.get(result), $.index, ($$anchor, item) => {
			var li = root();
			var text = $.child(li, true);
			$.reset(li);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, li);
		}), "each", App, 3, 2);
		$.reset(ul);
		$.append($$anchor, ul);
	}), "await", App, 1, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
