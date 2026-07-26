import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p></p>`, 1);
export default function App($$anchor) {
	let gate = $.state(0);
	let before = 1;
	var loaded;
	var $$promises = $.run([async () => loaded = await $.async_derived(() => $.get(gate))]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	p.textContent = "1";
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
