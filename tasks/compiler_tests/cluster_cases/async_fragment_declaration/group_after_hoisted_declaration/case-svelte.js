import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span> </span></div>`);
export default function App($$anchor) {
	const id = "name";
	var div = root();
	{
		const nested = "nested";
		let greeting2;
		var promises = $.run([async () => greeting2 = await $.async_derived(() => `Hi ${id}`)]);
		var span = $.child(div);
		var text = $.child(span);
		$.reset(span);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `nested ${$.get(greeting2) ?? ""}`), void 0, void 0, [promises[0]]);
	}
	$.append($$anchor, div);
}
