import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const before = $.derived(() => "sync");
			let awaited;
			let after;
			var promises = $.run([async () => awaited = (await $.save($.async_derived(async () => (await $.save(Promise.resolve($.get(n))))())))(), () => after = $.derived(() => $.get(awaited) + 1)]);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(before) ?? ""}${$.get(awaited) ?? ""}${$.get(after) ?? ""}`), void 0, void 0, [promises[0], promises[1]]);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		});
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
