import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { numbers } from "./data.js";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => ($.deep_read_state(numbers), $.untrack(() => numbers.join(" + ")))]);
	$.append($$anchor, p);
	$.pop();
}
