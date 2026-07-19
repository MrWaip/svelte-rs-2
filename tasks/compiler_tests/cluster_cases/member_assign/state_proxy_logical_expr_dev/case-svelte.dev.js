App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let object = $.tag_proxy($.proxy({ items: null }), "object");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `items: ${$0 ?? ""}`), [() => JSON.stringify(object.items)]);
	$.delegated("click", button, function click() {
		return $.assign(object, "items", "??=", () => [], "(unknown):5:24").push(object.items.length);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
