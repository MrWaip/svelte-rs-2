import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>+</button>`, 1);
export default function App($$anchor) {
	let log = [];
	function add() {
		log.push(1);
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, `v ${log ?? ""}`));
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
