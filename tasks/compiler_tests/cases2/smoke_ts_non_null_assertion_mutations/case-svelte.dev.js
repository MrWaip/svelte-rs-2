App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <button>run</button>`, 1), App[$.FILENAME], [[29, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ field: { x: 0 } }), "obj");
	let deep = $.tag_proxy($.proxy({ a: { b: { c: { x: 0 } } } }), "deep");
	function script_ops() {
		obj.field.x = 1;
		obj.field.x += 2;
		obj.field.x++;
		++obj.field.x;
		obj.field.x &&= 3;
		deep.a.b.c.x = 4;
		deep.a.b.c.x += 5;
		deep.a.b.c.x++;
	}
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, `${obj.field.x ?? ""}
${deep.a.b.c.x ?? ""}

${(obj.field.x = 1) ?? ""}
${(obj.field.x += 2) ?? ""}
${obj.field.x++ ?? ""}
${++obj.field.x ?? ""}

${(deep.a.b.c.x = 4) ?? ""}
${deep.a.b.c.x++ ?? ""} `));
	$.delegated("click", button, script_ops);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
