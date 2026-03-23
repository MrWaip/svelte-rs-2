import * as $ from "svelte/internal/client";
import { realValue } from "./utils";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let data = $.proxy({ value: 0 });
	function process(input) {
		return realValue.transform(input);
	}
	var button = root();
	button.textContent = realValue.label;
	$.delegated("click", button, () => process(data));
	$.append($$anchor, button);
}
$.delegate(["click"]);
