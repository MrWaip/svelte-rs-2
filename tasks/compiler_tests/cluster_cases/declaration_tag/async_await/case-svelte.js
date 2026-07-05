import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, item) => {
		let value;
		var promises = $.run([async () => value = await item.load()]);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, value), void 0, void 0, [promises[0]]);
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
