import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var value;
	var $$promises = $.run([async () => value = await $$props.promise]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
}
