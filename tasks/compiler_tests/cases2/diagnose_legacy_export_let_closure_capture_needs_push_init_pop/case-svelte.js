import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let duration = $.prop($$props, "duration", 8);
	const opts = () => ({ duration: duration() });
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.untrack(() => opts().duration)]);
	$.append($$anchor, p);
	$.pop();
}
