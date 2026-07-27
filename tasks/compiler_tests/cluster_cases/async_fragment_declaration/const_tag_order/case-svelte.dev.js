import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(1), "n");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const before = $.tag($.derived(() => "sync"), "before");
			$.get(before);
			let awaited;
			let after;
			var promises = $.run([async () => awaited = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve($.get(n))))())))(), "awaited"), () => after = $.tag($.derived(() => $.get(awaited) + 1), "after")]);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(before) ?? ""}${$.get(awaited) ?? ""}${$.get(after) ?? ""}`), void 0, void 0, [promises[0], promises[1]]);
			$.append($$anchor, p);
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
