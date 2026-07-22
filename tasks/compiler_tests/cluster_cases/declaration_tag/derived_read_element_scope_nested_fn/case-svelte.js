import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	var div = root();
	{
		let dt = $.derived(() => [
			1,
			2,
			3
		]);
		var text = $.child(div);
		$.reset(div);
		$.template_effect(($0) => $.set_text(text, `${$.get(dt).length ?? ""}
	${$0 ?? ""}`), [() => $.get(dt).map((x) => x + $.get(dt).length)]);
	}
	$.append($$anchor, div);
}
