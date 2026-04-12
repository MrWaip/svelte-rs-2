import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let count = 1;
	var step = 2;
	function bump() {
		count += 1;
		step += 1;
	}
	$: doubled = count * 2;
	$: total = doubled + step;
	var button = root();
	button.textContent = `${doubled ?? ""}-${total ?? ""}`;
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
