import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.prop($$props, "foo", 8, 1);
	function getFoo() {
		return foo();
	}
	var $$exports = {
		get foo() {
			return foo();
		},
		getFoo
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, p);
	$.bind_prop($$props, "foo", foo);
	$.bind_prop($$props, "getFoo", getFoo);
	return $.pop($$exports);
}
