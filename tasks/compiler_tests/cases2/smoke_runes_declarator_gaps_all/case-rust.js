import * as $ from "svelte/internal/client";
var root = $.from_html(` <button>run</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let rawProp = $.prop($$props, "rawProp", 7), rawPropObj = $.prop($$props, "rawPropObj", 7);
	var safeCount = $.state(0);
	var safeObj = $.proxy({ x: 0 });
	function script_ops() {
		$.set(safeCount, 1);
		$.set(safeCount, $.safe_get(safeCount) + 2);
		$.update(safeCount);
		$.update_pre(safeCount);
		$.set(safeCount, $.safe_get(safeCount) && 3);
		safeObj.x = 1;
		safeObj.x += 2;
		safeObj.x++;
		rawProp(8);
		rawProp(rawProp() + 9);
		$.update_prop(rawProp);
		$.update_pre_prop(rawProp);
		rawProp(rawProp() && 10);
		rawPropObj().x = 11;
		rawPropObj().x += 12;
		rawPropObj().x++;
	}
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, `${$.safe_get(safeCount) ?? ""}
${safeObj.x ?? ""}
${rawProp() ?? ""}
${rawPropObj().x ?? ""}

${$.set(safeCount, 1) ?? ""}
${$.set(safeCount, $.safe_get(safeCount) + 2) ?? ""}
${$.update(safeCount) ?? ""}
${$.update_pre(safeCount) ?? ""}
${$.set(safeCount, $.safe_get(safeCount) && 3) ?? ""}

${(safeObj.x = 1) ?? ""}
${(safeObj.x += 2) ?? ""}
${safeObj.x++ ?? ""}

${rawProp(8) ?? ""}
${rawProp(rawProp() + 9) ?? ""}
${$.update_prop(rawProp) ?? ""}
${rawProp(rawProp() && 10) ?? ""}

${(rawPropObj().x = 11) ?? ""}
${(rawPropObj().x += 12) ?? ""}
${rawPropObj().x++ ?? ""} `));
	$.delegated("click", button, script_ops);
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
