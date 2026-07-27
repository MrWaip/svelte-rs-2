import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>toggle</button> <!doctype/>`, 1);
export default function App($$anchor) {
	let kind = $.state("html");
	var fragment = root();
	var button = $.first_child(fragment);
	var _doctype = $.sibling(button, 2);
	$.template_effect(() => $.set_attribute(_doctype, "html", $.get(kind)));
	$.delegated("click", button, () => $.set(kind, "xml"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
