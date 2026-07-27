import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>inc</button>`, 1);
export default function App($$anchor) {
	let count = 0;
	const promise = Promise.resolve(1);
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let value;
			var promises = $.run([async () => value = (await $.save($.async_derived(async () => (await $.save(promise))())))()]);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(value) ?? ""} ${count ?? ""}`), void 0, void 0, [promises[0]]);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (count >= 0) $$render(consequent);
		});
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => count++);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
