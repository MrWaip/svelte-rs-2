App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[3, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => items, $.index, ($$anchor, item) => {
		let current = item;
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, current));
		$.delegated("click", button, function click() {
			return current++;
		});
		$.append($$anchor, button);
	}), "each", App, 1, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
