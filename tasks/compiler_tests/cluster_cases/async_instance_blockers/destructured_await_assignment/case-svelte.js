import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor) {
	let gate = $.state(0);
	var first, second;
	var $$promises = $.run([async () => ({first, second} = await Promise.resolve({
		first: $.get(gate),
		second: 2
	}))]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${first ?? ""}${second ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
