import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<p>empty</p>`);
export default function App($$anchor) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded;
	var $$promises = $.run([async () => loaded = await delay([1, 2])]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [$$promises[0]], void 0, (node) => {
		$.each(node, 17, () => loaded, $.index, ($$anchor, item) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p);
		}, ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		});
	});
	$.append($$anchor, fragment);
}
