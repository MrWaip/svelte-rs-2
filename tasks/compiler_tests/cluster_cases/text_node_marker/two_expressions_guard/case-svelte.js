import * as $ from "svelte/internal/client";
var root = $.from_html(` <button>inc</button>`, 1);
export default function App($$anchor) {
	let a = $.state(0);
	let b = $.state(0);
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, () => {
		$.update(a);
		$.update(b);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
