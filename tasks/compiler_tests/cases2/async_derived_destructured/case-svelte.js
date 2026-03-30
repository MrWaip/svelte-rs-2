import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let url = "/api";
	var data, meta;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(() => fetch(url).then((r) => r.json()));
		data = $.derived(() => $.get($$d).data);
		meta = $.derived(() => $.get($$d).meta);
	}]);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(data) ?? ""}-${$.get(meta) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
	$.pop();
}
