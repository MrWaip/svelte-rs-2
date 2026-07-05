App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let obj = $.prop($$props, "obj", 23, () => ({ x: 0 }));
	obj().x;
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $$ownership_validator.mutation("obj", ["obj", "x"], obj().x += 5, 5, 1)));
	$.append($$anchor, text);
	return $.pop($$exports);
}
