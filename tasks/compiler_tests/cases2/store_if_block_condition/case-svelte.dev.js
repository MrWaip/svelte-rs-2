App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { cond } from "./stores";
var root = $.add_locations($.from_html(`<p>visible</p>`), App[$.FILENAME], [[5, 11]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $cond = () => ($.validate_store(cond, "cond"), $.store_get(cond, "$cond", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($cond()) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
