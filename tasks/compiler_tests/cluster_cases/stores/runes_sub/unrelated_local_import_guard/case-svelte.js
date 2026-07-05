import * as $ from "svelte/internal/client";
import { helper } from "./store.js";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${helper ?? ""}`));
	$.delegated("click", button, () => $.update(a));
	$.append($$anchor, button);
}
$.delegate(["click"]);
