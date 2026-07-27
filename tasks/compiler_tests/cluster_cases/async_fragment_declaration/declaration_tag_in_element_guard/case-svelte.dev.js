import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><p> </p></div> <button>go</button>`, 1), App[$.FILENAME], [[
	5,
	0,
	[[7, 1]]
], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(1), "n");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	{
		let a;
		var promises = $.run([async () => a = (await $.track_reactivity_loss(Promise.resolve($.get(n))))()]);
		var p = $.child(div);
		var text = $.child(p, true);
		$.reset(p);
		$.reset(div);
		$.template_effect(() => $.set_text(text, a), void 0, void 0, [promises[0]]);
	}
	var button = $.sibling(div, 2);
	$.delegated("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
