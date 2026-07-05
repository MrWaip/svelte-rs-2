App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p> <p></p>`, 1), App[$.FILENAME], [[19, 0], [20, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	let status = "active";
	function greet(user) {
		return `Hello ${user.name}`;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	p.textContent = "world";
	var p_1 = $.sibling(p, 2);
	p_1.textContent = "active";
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
