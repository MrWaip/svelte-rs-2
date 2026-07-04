import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { Kind } from "./kinds";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let item = $.prop($$props, "item", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(() => $.set_text(text, `Prefix ${($.deep_read_state(item()), $.deep_read_state(Kind), $.untrack(() => $.strict_equals(item()?.kind, Kind.A) ? "one" : "two")) ?? ""} suffix`));
	$.append($$anchor, span);
	return $.pop($$exports);
}
