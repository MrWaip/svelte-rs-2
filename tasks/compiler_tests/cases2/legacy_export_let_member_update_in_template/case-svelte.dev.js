import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let obj = $.prop($$props, "obj", 28, () => ({ x: 0 }));
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x++, true), 5, 1)))));
	$.append($$anchor, text);
	return $.pop($$exports);
}
