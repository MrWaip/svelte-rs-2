import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $foo = () => ($.validate_store(foo, "foo"), $.store_get(foo, "$foo", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let foo = writable(42);
	let cond = true;
	let x = $.prop($$props, "x", 24, () => cond ? $foo() : 0);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, x()));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
