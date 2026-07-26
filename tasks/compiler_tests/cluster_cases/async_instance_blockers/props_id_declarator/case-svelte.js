import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor) {
	const uid = $.props_id();
	let gate = $.state(0);
	var loaded, after;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate)), () => after = $.get(gate) + 1]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${uid}${after}`), void 0, void 0, [$$promises[1]]);
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
