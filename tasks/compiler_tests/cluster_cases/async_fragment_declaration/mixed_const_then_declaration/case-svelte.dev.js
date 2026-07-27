import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let a;
			let b;
			let c;
			var promises = $.run([async () => a = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve(n)))())))(), "a"), () => {
				b = $.tag($.derived(() => $.get(a) * 2), "b");
				c = $.get(a) + 1;
			}]);
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `${$.get(b) ?? ""} ${c}`), void 0, void 0, [promises[1]]);
			$.append($$anchor, span);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (n) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
