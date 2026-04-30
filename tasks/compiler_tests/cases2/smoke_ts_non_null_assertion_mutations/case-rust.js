import * as $ from "svelte/internal/client";
var root = $.from_html(` <button>run</button>`, 1);
export default function App($$anchor) {
	let obj = $.proxy({ field: { x: 0 } });
	let deep = $.proxy({ a: { b: { c: { x: 0 } } } });
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
}
$.delegate(["click"]);
