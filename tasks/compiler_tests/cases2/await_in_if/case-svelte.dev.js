App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	const promise = fetch("/api");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.await(node_1, () => promise, null, ($$anchor, value) => {
				var p = root();
				var text = $.child(p, true);
				$.reset(p);
				$.template_effect(() => $.set_text(text, $.get(value)));
				$.append($$anchor, p);
			}), "await", App, 7, 1);
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
