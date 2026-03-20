import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p> <p></p>`, 1);
export default function App($$anchor) {
	let name = "world";
	let status = "active";
	function greet(user) {
		return `Hello ${user.name}`;
	}
	var fragment = root();
	var p = $.first_child(fragment);
	p.textContent = "world";
	var p_1 = $.sibling(p, 2);
	p_1.textContent = "active";
	$.append($$anchor, fragment);
}
