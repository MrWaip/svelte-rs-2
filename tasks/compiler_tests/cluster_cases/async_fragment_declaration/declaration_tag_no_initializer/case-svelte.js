import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = 1;
	let a;
	let b;
	var promises = $.run([async () => a = await Promise.resolve(n), () => {}]);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}`), void 0, void 0, [promises[0], promises[1]]);
	$.delegated("click", button, () => b = n);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
