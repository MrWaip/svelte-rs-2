import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>inc</button>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var value;
	var $$promises = $.run([async () => value = (await $.track_reactivity_loss(Promise.resolve(1)))()]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${value ?? ""} ${count ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, function click() {
		return count++;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
