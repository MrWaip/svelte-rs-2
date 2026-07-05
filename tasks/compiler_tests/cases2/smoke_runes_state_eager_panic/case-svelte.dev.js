App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <button>run</button>`, 1), App[$.FILENAME], [[20, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function script_ops() {
		eager = 1;
		eager++;
		eagerObj.x = 2;
		eagerObj.x++;
	}
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, `${eager ?? ""}
${eagerObj.x ?? ""}
${(eager = 3) ?? ""}
${eager++ ?? ""}
${(eagerObj.x = 4) ?? ""}
${eagerObj.x++ ?? ""} `));
	$.delegated("click", button, script_ops);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
