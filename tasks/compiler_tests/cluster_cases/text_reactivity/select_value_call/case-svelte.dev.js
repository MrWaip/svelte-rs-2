App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select></select>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function test() {}
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var select_value;
	$.init_select(select);
	$.template_effect(($0) => {
		if (select_value !== (select_value = $0)) {
			select.value = (select.__value = $0) ?? "", $.select_option(select, $0);
		}
	}, [() => test()]);
	$.append($$anchor, select);
	return $.pop($$exports);
}
