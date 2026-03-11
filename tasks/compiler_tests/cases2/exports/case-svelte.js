import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const PI = 3.14;
	function greet(name) {
		return "Hello " + name;
	}
	var $$exports = {
		PI,
		greet
	};
	var p = root();
	p.textContent = "PI is 3.14";
	$.append($$anchor, p);
	return $.pop($$exports);
}
