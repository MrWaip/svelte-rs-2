import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { Kind } from "./kinds";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let item = $.prop($$props, "item", 8);
	$.init();
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(() => $.set_text(text, `Prefix ${($.deep_read_state(item()), $.deep_read_state(Kind), $.untrack(() => item()?.kind === Kind.A ? "one" : "two")) ?? ""} suffix`));
	$.append($$anchor, span);
	$.pop();
}
