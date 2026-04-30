import * as $ from "svelte/internal/client";
var root = $.from_html(` <button>run</button>`, 1);
export default function App($$anchor) {
	function script_ops() {
		eager = 1;
		eager++;
		eagerObj.x = 2;
		eagerObj.x++;
	}
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
}
$.delegate(["click"]);
