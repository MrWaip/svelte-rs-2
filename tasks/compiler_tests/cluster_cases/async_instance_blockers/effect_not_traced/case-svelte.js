import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let gate = $.state(0);
	let shown = 1;
	var loaded;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate)), () => void $.user_effect(() => {
		shown = 2;
	})]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, shown));
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
