import { writable } from "svelte/store";
import * as $ from "svelte/internal/client";
export const theme = writable("light");
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
