import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>update</button> `, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var value;
	var $$promises = $.run([() => Promise.resolve(), () => value = $.prop($$props, "value", 15, "test")]);
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(() => $.set_text(text, ` ${value() ?? ""}`), void 0, void 0, [$$promises[1]]);
	$.delegated("click", button, () => value("updated"));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
