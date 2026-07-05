App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <button>run</button>`, 1), App[$.FILENAME], [[54, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let rawProp = $.prop($$props, "rawProp", 7), rawPropObj = $.prop($$props, "rawPropObj", 7);
	var safeCount = $.tag($.state(0), "safeCount");
	var safeObj = $.tag_proxy($.proxy({ x: 0 }), "safeObj");
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
		$$ownership_validator.mutation("rawPropObj", ["rawPropObj", "x"], rawPropObj().x = 11, 24, 2);
		$$ownership_validator.mutation("rawPropObj", ["rawPropObj", "x"], rawPropObj().x += 12, 25, 2);
		$$ownership_validator.mutation("rawPropObj", ["rawPropObj", "x"], rawPropObj().x++, 26, 2);
	}
	var $$exports = { ...$.legacy_api() };
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

${$$ownership_validator.mutation("rawPropObj", ["rawPropObj", "x"], rawPropObj().x = 11, 50, 1) ?? ""}
${$$ownership_validator.mutation("rawPropObj", ["rawPropObj", "x"], rawPropObj().x += 12, 51, 1) ?? ""}
${$$ownership_validator.mutation("rawPropObj", ["rawPropObj", "x"], rawPropObj().x++, 52, 1) ?? ""} `));
	$.delegated("click", button, script_ops);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
