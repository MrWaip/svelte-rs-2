import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>update</button> `, 1), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var value;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => value = $.prop($$props, "value", 15, "test")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(() => $.set_text(text, ` ${value() ?? ""}`), void 0, void 0, [$$promises[1]]);
	$.delegated("click", button, function click() {
		return value("updated");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
