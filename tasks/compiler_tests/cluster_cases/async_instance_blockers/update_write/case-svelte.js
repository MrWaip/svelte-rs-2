import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p> <p> </p>`, 1);
export default function App($$anchor) {
	let gate = $.state(0);
	let counter = 1;
	var loaded;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate)), () => void counter++]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, counter);
		$.set_text(text_1, $.get(loaded));
	}, void 0, void 0, [$$promises[1], $$promises[0]]);
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
