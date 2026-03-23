import * as $ from "svelte/internal/client";
import { realValue } from "./utils";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let data = $.proxy({ value: 0 });
	function process(input) {
		return realValue.transform(input);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, realValue.label));
	$.delegated("click", button, () => process(data));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
