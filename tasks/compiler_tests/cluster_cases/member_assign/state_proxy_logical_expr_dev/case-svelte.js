import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let object = $.proxy({ items: null });
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `items: ${$0 ?? ""}`), [() => JSON.stringify(object.items)]);
	$.delegated("click", button, () => (object.items ??= []).push(object.items.length));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
