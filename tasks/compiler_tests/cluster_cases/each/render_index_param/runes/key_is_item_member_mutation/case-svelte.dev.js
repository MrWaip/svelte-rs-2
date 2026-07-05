App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{ name: "a" }]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => items, (item) => item, ($$anchor, item, $$index) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, item.name));
		$.delegated("click", button, function click() {
			return item.name += "!";
		});
		$.append($$anchor, button);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
