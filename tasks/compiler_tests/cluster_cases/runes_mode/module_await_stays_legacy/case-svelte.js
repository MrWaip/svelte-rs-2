import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
const shared = await Promise.resolve(1);
var root = $.from_html(`<p> </p> <button>inc</button>`, 1);
export default function App($$anchor) {
	let count = $.mutable_source(0);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `${shared ?? ""} ${$.get(count) ?? ""}`));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
