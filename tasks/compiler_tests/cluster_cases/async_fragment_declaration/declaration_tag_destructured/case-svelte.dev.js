import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>go</button>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let source = $.tag($.state($.proxy({
		x: 1,
		y: 2
	})), "source");
	var $$exports = { ...$.legacy_api() };
	let x;
	let y;
	var promises = $.run([async () => ({x, y} = (await $.track_reactivity_loss(Promise.resolve($.get(source))))())]);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${x ?? ""}${y ?? ""}`), void 0, void 0, [promises[0]]);
	$.delegated("click", button, function click() {
		return $.set(source, {
			x: 3,
			y: 4
		}, true);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
