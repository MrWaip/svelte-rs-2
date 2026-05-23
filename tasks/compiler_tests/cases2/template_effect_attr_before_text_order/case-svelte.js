import * as $ from "svelte/internal/client";
import { f, g } from "./x";
var root = $.from_html(`<a> </a>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let url = $.prop($$props, "url", 3, ""), label = $.prop($$props, "label", 3, "");
	var a = root();
	var text = $.child(a, true);
	$.reset(a);
	$.template_effect(($0, $1) => {
		$.set_attribute(a, "href", $0);
		$.set_text(text, $1);
	}, [() => f(url()), () => g(label())]);
	$.append($$anchor, a);
	$.pop();
}
