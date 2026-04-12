import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import data from "./dep.js";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	function bump() {
		data.count += 1;
	}
	$: doubled = total * 2;
	$: total = data.count;
	var button = root();
	button.textContent = doubled;
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
