App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"a"
]);
var root = $.add_locations($.from_html(`<p>foo exists</p>`), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<p> </p> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$slots = $.sanitize_slots($$props);
	$.push($$props, true, App);
	let rest = $.rest_props($$props, rest_excludes, "rest");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var node = $.sibling(p, 2);
	{
		var consequent = ($$anchor) => {
			var p_1 = root();
			$.append($$anchor, p_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$slots.foo) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.template_effect(($0) => $.set_text(text, `${$$props.a ?? ""} ${$0 ?? ""}`), [() => Object.keys(rest)]);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
