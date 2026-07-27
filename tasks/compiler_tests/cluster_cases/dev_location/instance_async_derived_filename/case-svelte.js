import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var total;
	var $$promises = $.run([async () => total = await $.async_derived(() => $$props.p)]);
	var p_1 = root();
	var text = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => $.set_text(text, $.get(total)), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p_1);
}
