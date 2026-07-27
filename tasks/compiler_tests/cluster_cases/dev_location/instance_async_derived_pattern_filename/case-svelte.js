import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var first, second;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(() => $$props.p);
		first = $.derived(() => $.get($$d).first);
		second = $.derived(() => $.get($$d).second);
	}]);
	var p_1 = root();
	var text = $.child(p_1);
	$.reset(p_1);
	$.template_effect(() => $.set_text(text, `${$.get(first) ?? ""} ${$.get(second) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p_1);
}
