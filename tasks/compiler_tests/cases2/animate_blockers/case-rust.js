import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let items = [{
		id: 1,
		name: "a"
	}];
	var data, params;
	var $$promises = $.run([async () => data = await fetch("/api"), () => params = $.proxy(data.params)]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 25, () => items, (item) => item.id, ($$anchor, item) => {
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.run_after_blockers([$$promises[1]], () => {
			$.animation(div, () => flip, () => params);
		});
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
