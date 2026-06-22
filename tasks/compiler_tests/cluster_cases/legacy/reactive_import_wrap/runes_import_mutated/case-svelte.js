import * as $ from "svelte/internal/client";
import foo from "./foo.js";
var root = $.from_html(`<p> </p> <button>add</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	function add() {
		foo.bar = "baz";
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, foo.bar));
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
