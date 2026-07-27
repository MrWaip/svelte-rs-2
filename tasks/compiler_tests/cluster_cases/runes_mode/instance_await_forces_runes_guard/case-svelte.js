import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>inc</button>`, 1);
export default function App($$anchor) {
	let count = 0;
	var value;
	var $$promises = $.run([async () => value = await Promise.resolve(1)]);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${value ?? ""} ${count ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, () => count++);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
