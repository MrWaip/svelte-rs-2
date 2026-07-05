App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const PI = 3.14;
	function greet(name) {
		return "Hello " + name;
	}
	var $$exports = {
		...$.legacy_api(),
		get PI() {
			return PI;
		},
		get greet() {
			return greet;
		}
	};
	var p = root();
	p.textContent = "PI is 3.14";
	$.append($$anchor, p);
	return $.pop($$exports);
}
