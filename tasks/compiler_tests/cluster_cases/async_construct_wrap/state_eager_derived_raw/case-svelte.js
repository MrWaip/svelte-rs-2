import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var delayed, doubled;
	var $$promises = $.run([async () => delayed = await $.async_derived(() => Promise.resolve($.get(count))), () => doubled = $.derived(() => $.get(count) * 2)]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `${$.get(delayed) ?? ""}${$0 ?? ""}`), [() => $.eager(() => $.get(doubled)) !== $.get(doubled)], void 0, [$$promises[0], $$promises[1]]);
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
