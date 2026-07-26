import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><span> </span></div>`), App[$.FILENAME], [[
	7,
	1,
	[[9, 2]]
]]);
var root_1 = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(1), "n");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let outer;
			var promises = $.run([async () => outer = await $.async_derived(async () => (await $.track_reactivity_loss(Promise.resolve($.get(n))))(), "outer", "(unknown):6:14")]);
			var div = root();
			{
				let inner;
				var promises_1 = $.run([() => promises[0].promise, () => inner = $.tag($.derived(() => `v${$.get(outer)}`), "inner")]);
				var span = $.child(div);
				var text = $.child(span, true);
				$.reset(span);
				$.reset(div);
				$.template_effect(() => $.set_text(text, $.get(inner)), void 0, void 0, [promises_1[1]]);
			}
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		}), "if", App, 5, 0);
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
