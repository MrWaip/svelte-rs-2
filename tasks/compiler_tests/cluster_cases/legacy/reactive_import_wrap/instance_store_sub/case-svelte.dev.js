import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import foo from "./foo.js";
var $$_import_foo = $.reactive_import(() => foo);
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $foo = () => ($.validate_store($$_import_foo(), "foo"), $.store_get($$_import_foo(), "$foo", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	$$_import_foo($$_import_foo().bar = "baz");
	const answer = $foo();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, answer));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
