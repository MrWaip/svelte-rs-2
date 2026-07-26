import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let items = $.proxy([1, 2]);
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		let loaded;
		var promises = $.run([async () => loaded = (await $.save($.async_derived(async () => (await $.save(Promise.resolve($.get(item))))())))()]);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(loaded)), void 0, void 0, [promises[0]]);
		$.append($$anchor, p);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => items.push(3));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
