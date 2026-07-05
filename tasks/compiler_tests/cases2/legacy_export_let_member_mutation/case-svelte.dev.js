import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let obj = $.prop($$props, "obj", 28, () => ({ a: 1 }));
	$$ownership_validator.mutation(null, ["obj", "a"], obj(obj().a = 99, true), 4, 1);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(obj()), $.untrack(() => obj().a))));
	$.append($$anchor, p);
	return $.pop($$exports);
}
