import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>go</button>`, 1);
export default function App($$anchor) {
	let source = $.state($.proxy({
		x: 1,
		y: 2
	}));
	let x;
	let y;
	var promises = $.run([async () => ({x, y} = await Promise.resolve($.get(source)))]);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${x ?? ""}${y ?? ""}`), void 0, void 0, [promises[0]]);
	$.delegated("click", button, () => $.set(source, {
		x: 3,
		y: 4
	}, true));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
