import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const shared = (await $.track_reactivity_loss(Promise.resolve(1)))();
var root = $.add_locations($.from_html(`<p> </p> <button>inc</button>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.tag($.mutable_source(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${shared ?? ""} ${$.get(count) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
