import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let obj = $.prop($$props, "obj", 15);
	function sync() {
		obj(obj().field = obj().other, true);
	}
	var button = root();
	$.delegated("click", button, sync);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
