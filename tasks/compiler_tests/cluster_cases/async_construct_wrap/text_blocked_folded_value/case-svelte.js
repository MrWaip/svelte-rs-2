import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor) {
	let count = $.state(1);
	async function getDouble(value) {
		return value * 2;
	}
	var double;
	var $$promises = $.run([async () => double = await $.async_derived(() => getDouble($.get(count)))]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""} Double: ${$.get(double) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
