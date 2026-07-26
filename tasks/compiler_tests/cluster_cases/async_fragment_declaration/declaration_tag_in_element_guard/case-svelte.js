import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><p> </p></div> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	var fragment = root();
	var div = $.first_child(fragment);
	{
		let a;
		var promises = $.run([async () => a = await Promise.resolve($.get(n))]);
		var p = $.child(div);
		var text = $.child(p, true);
		$.reset(p);
		$.reset(div);
		$.template_effect(() => $.set_text(text, a), void 0, void 0, [promises[0]]);
	}
	var button = $.sibling(div, 2);
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
