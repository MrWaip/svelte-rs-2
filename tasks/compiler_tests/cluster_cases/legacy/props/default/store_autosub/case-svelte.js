import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $foo = () => $.store_get(foo, "$foo", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let foo = writable(42);
	let initial_foo = $.prop($$props, "initial_foo", 24, $foo);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, initial_foo()));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
