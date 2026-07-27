App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	const g1 = $.tag($.derived(() => $.get(a) && (() => $.get(a))), "g1");
	const g2 = $.tag($.derived(() => $.get(a) ?? (() => $.get(a))), "g2");
	const g3 = $.tag($.derived(() => () => ({ x: $.get(a) })), "g3");
	const g4 = $.tag($.derived(() => ($.get(a), () => $.get(a))), "g4");
	const g5 = $.tag($.derived(() => ($.get(a) + 1) * 2), "g5");
	const g6 = $.tag($.derived(() => (() => $.get(a))()), "g6");
	const g7 = $.tag($.derived(() => (function() {
		return $.get(a);
	})()), "g7");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0, $1) => $.set_text(text, `${$.get(g5) ?? ""} ${$.get(g6) ?? ""} ${$.get(g7) ?? ""} ${typeof $.get(g1)} ${typeof $.get(g2)} ${$0 ?? ""} ${$1 ?? ""}`), [() => $.get(g3)().x, () => $.get(g4)()]);
	$.delegated("click", button, function click() {
		return $.update(a);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
