import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>go</button>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = 1;
	var $$exports = { ...$.legacy_api() };
	let a;
	let b;
	var promises = $.run([async () => a = (await $.track_reactivity_loss(Promise.resolve(n)))(), () => {}]);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}`), void 0, void 0, [promises[0], promises[1]]);
	$.delegated("click", button, function click() {
		return b = n;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
