App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import foo from "./foo.js";
var root = $.add_locations($.from_html(`<p> </p> <button>add</button>`, 1), App[$.FILENAME], [[8, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function add() {
		foo.bar = "baz";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, foo.bar));
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
