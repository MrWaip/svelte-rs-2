import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var name, age;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(() => fetch("/api"));
		name = $.derived(() => $.get($$d).name);
		age = $.derived(() => $.get($$d).age);
	}]);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""} ${$.get(age) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
}
