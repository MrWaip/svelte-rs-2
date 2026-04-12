import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>add</button>`, 1);
export default function App($$anchor) {
	let numbers = [
		1,
		2,
		3
	];
	function add() {
		numbers.push(numbers.length + 1);
		numbers = numbers;
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, numbers.length));
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
