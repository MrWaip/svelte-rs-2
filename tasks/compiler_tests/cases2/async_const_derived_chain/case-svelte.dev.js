import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var data;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))()]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let a;
			let b;
			let c;
			var promises = $.run([
				() => $$promises[0].promise,
				() => a = $.tag($.derived(() => data.value), "a"),
				() => b = $.tag($.derived(() => $.get(a) * 2), "b"),
				() => c = $.tag($.derived(() => $.get(b) + 1), "c")
			]);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(c)), void 0, void 0, [promises[3]]);
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
