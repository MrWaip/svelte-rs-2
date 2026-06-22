import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let foo = 1;
	var $$exports = {
		get "foo-bar"() {
			return foo;
		},
		set "foo-bar"($$value) {
			foo = $$value;
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, foo));
	$.append($$anchor, p);
	return $.pop($$exports);
}
