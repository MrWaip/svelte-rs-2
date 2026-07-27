import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	var d, e;
	var $$promises = $.run([async () => d = await $.async_derived(() => Promise.resolve($.get(n))), async () => e = await $.async_derived(() => Promise.resolve($.get(n) + 1))]);
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.async(node, [$$promises[0]], void 0, (node) => {
		var consequent = ($$anchor) => {
			let v;
			var promises = $.run([() => $.wait([$$promises[0], $$promises[1]]), () => v = $.derived(() => $.get(d) + $.get(e))]);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(v)), void 0, void 0, [promises[1]]);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
		});
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
