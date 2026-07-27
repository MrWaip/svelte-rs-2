import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let arr = $.proxy([
		1,
		2,
		3
	]);
	var a, rest, x, others;
	var $$promises = $.run([() => Promise.resolve(), () => {
		var $$array = $.derived(() => $.to_array(arr));
		a = $.derived(() => $.get($$array)[0]);
		rest = $.derived(() => $.get($$array).slice(1));
		var $$d = $.derived(() => ({ x: 2 }));
		x = $.derived(() => $.fallback($.get($$d).x, 1));
		others = $.derived(() => $.exclude_from_object($.get($$d), ["x"]));
	}]);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${$.get(rest).length ?? ""} ${$.get(x) ?? ""} ${$.get(others).y ?? ""}`), void 0, void 0, [$$promises[1], $$promises[1]]);
	$.append($$anchor, p);
}
