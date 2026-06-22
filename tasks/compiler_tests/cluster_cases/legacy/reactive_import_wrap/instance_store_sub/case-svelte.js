import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import foo from "./foo.js";
var $$_import_foo = $.reactive_import(() => foo);
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $foo = () => $.store_get($$_import_foo(), "$foo", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	$$_import_foo($$_import_foo().bar = "baz");
	const answer = $foo();
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, answer));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
