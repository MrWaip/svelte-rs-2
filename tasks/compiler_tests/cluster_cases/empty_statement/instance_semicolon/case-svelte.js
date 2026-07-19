import * as $ from "svelte/internal/client";
import { noop } from "./x.js";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = 0;
	noop(count);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, count));
	$.delegated("click", button, () => count++);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
