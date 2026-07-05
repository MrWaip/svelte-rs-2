App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(` <button>snippet</button>`, 1), App[$.FILENAME], [[382, 1]]);
var root_1 = $.add_locations($.from_html(` <button>each-row</button>`, 1), App[$.FILENAME], [[354, 1]]);
var root_2 = $.add_locations($.from_html(` <!> <!> <!> <button>run</button>`, 1), App[$.FILENAME], [[430, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	const $store = () => ($.validate_store(store, "store"), $.store_get(store, "$store", $$stores));
	const $objStore = () => ($.validate_store(objStore, "objStore"), $.store_get(objStore, "$objStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const card = $.wrap_snippet(App, function($$anchor, param = $.noop) {
		$.validate_snippet_args(...arguments);
		$.next();
		var fragment = root();
		var text = $.first_child(fragment);
		var button = $.sibling(text);
		$.template_effect(() => $.set_text(text, `${param() ?? ""}
	${param().x ?? ""}
	${(param().x = 1) ?? ""}
	${(param().x += 2) ?? ""}
	${param().x++ ?? ""}
	${++param().x ?? ""}
	${(param().x &&= 3) ?? ""}
	${(param().x ||= 4) ?? ""}
	${(param().x ??= 5) ?? ""}
	${(param()["x"] = 1) ?? ""}
	${param()["x"]++ ?? ""}
	${(param()[key] = 1) ?? ""}
	${param()[key]++ ?? ""} `));
		$.delegated("click", button, function click() {
			param().x = 1;
			param().x += 2;
			param().x++;
			++param().x;
			param().x &&= 3;
			param().x ||= 4;
			param().x ??= 5;
			param()[key] = 1;
			param()[key]++;
		});
		$.append($$anchor, fragment);
	});
	let prop = $.prop($$props, "prop", 7, 0), propObj = $.prop($$props, "propObj", 23, () => ({ x: 0 })), bind = $.prop($$props, "bind", 15, 0), bindObj = $.prop($$props, "bindObj", 31, () => $.tag_proxy($.proxy({ x: 0 }), "bindObj"));
	let count = $.tag($.state(0), "count");
	let countObj = $.tag_proxy($.proxy({ x: 0 }), "countObj");
	let raw = $.tag($.state(0), "raw");
	let deep = $.tag_proxy($.proxy({ a: { b: { c: { x: 0 } } } }), "deep");
	let key = "x";
	let nestedKey = "b";
	const store = writable(0);
	const objStore = writable({ x: 0 });
	let items = $.tag_proxy($.proxy([{
		id: 1,
		x: 0
	}, {
		id: 2,
		x: 0
	}]), "items");
	let promise = Promise.resolve({ x: 0 });
	function script_ops() {
		$.set(count, 1);
		$.set(count, $.get(count) + 2);
		$.set(count, $.get(count) - 3);
		$.update(count);
		$.update(count, -1);
		$.update_pre(count);
		$.update_pre(count, -1);
		$.set(count, $.get(count) && 4);
		$.set(count, $.get(count) || 5);
		$.set(count, $.get(count) ?? 6);
		countObj.x = 7;
		countObj.x += 8;
		countObj.x++;
		countObj.x--;
		++countObj.x;
		--countObj.x;
		countObj.x &&= 9;
		countObj.x ||= 10;
		countObj.x ??= 11;
		countObj["x"] = 7;
		countObj["x"] += 8;
		countObj["x"]++;
		++countObj["x"];
		countObj[key] = 7;
		countObj[key] += 8;
		countObj[key]++;
		++countObj[key];
		deep.a.b.c.x = 1;
		deep.a.b.c.x += 2;
		deep.a.b.c.x -= 3;
		deep.a.b.c.x++;
		deep.a.b.c.x--;
		++deep.a.b.c.x;
		--deep.a.b.c.x;
		deep.a.b.c.x &&= 4;
		deep.a.b.c.x ||= 5;
		deep.a.b.c.x ??= 6;
		deep["a"]["b"]["c"]["x"] = 1;
		deep["a"]["b"]["c"]["x"] += 2;
		deep["a"]["b"]["c"]["x"]++;
		++deep["a"]["b"]["c"]["x"];
		deep[nestedKey].c[key] = 1;
		deep[nestedKey].c[key] += 2;
		deep[nestedKey].c[key]++;
		++deep[nestedKey].c[key];
		deep.a[nestedKey].c.x = 1;
		deep.a[nestedKey].c.x += 2;
		deep.a[nestedKey].c.x++;
		deep.a.b.c[key] = 1;
		deep.a.b.c[key]++;
		$.set(raw, 12);
		$.set(raw, $.get(raw) + 13);
		$.set(raw, $.get(raw) - 14);
		$.update(raw);
		$.update(raw, -1);
		$.update_pre(raw);
		$.update_pre(raw, -1);
		$.set(raw, $.get(raw) && 15);
		$.set(raw, $.get(raw) || 16);
		$.set(raw, $.get(raw) ?? 17);
		prop(18);
		prop(prop() + 19);
		$.update_prop(prop);
		$.update_prop(prop, -1);
		$.update_pre_prop(prop);
		$.update_pre_prop(prop, -1);
		prop(prop() && 20);
		prop(prop() || 21);
		prop(prop() ?? 22);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x = 23, 94, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x += 24, 95, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x++, 96, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x--, 97, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], ++propObj().x, 98, 4);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], --propObj().x, 99, 4);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x &&= 25, 100, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x ||= 26, 101, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x ??= 27, 102, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj()["x"] = 23, 103, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj()["x"] += 24, 104, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj()["x"]++, 105, 2);
		$$ownership_validator.mutation("propObj", ["propObj", "x"], ++propObj()["x"], 106, 4);
		$$ownership_validator.mutation("propObj", ["propObj", key], propObj()[key] = 23, 107, 2);
		$$ownership_validator.mutation("propObj", ["propObj", key], propObj()[key] += 24, 108, 2);
		$$ownership_validator.mutation("propObj", ["propObj", key], propObj()[key]++, 109, 2);
		$$ownership_validator.mutation("propObj", ["propObj", key], ++propObj()[key], 110, 4);
		bind(28);
		bind(bind() + 29);
		$.update_prop(bind);
		$.update_prop(bind, -1);
		$.update_pre_prop(bind);
		$.update_pre_prop(bind, -1);
		bind(bind() && 30);
		bind(bind() || 31);
		bind(bind() ?? 32);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x = 33, true), 122, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x += 34, true), 123, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x++, true), 124, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x--, true), 125, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(++bindObj().x, true), 126, 4);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(--bindObj().x, true), 127, 4);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x &&= 35, true), 128, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x ||= 36, true), 129, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x ??= 37, true), 130, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj()["x"] = 33, true), 131, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj()["x"] += 34, true), 132, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj()["x"]++, true), 133, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(++bindObj()["x"], true), 134, 4);
		$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(bindObj()[key] = 33, true), 135, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(bindObj()[key] += 34, true), 136, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(bindObj()[key]++, true), 137, 2);
		$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(++bindObj()[key], true), 138, 4);
		$.store_set(store, 38);
		$.store_set(store, $store() + 39);
		$.update_store(store, $store());
		$.update_store(store, $store(), -1);
		$.update_pre_store(store, $store());
		$.update_pre_store(store, $store(), -1);
		$.store_set(store, $store() && 40);
		$.store_set(store, $store() || 41);
		$.store_set(store, $store() ?? 42);
		$.store_mutate(objStore, $.untrack($objStore).x = 43, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x += 44, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x &&= 45, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ||= 46, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ??= 47, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] = 43, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] += 44, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] = 43, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] += 44, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore));
	}
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment_1 = root_2();
	var text_1 = $.first_child(fragment_1);
	var node = $.sibling(text_1);
	$.add_svelte_meta(() => $.each(node, 19, () => items, (item) => item.id, ($$anchor, item, i) => {
		$.next();
		var fragment_2 = root_1();
		var text_2 = $.first_child(fragment_2);
		var button_1 = $.sibling(text_2);
		$.template_effect(() => $.set_text(text_2, `${$.get(item) ?? ""}
	${$.get(i) ?? ""}
	${$.get(item).x ?? ""}
	${($.get(item).x = 1) ?? ""}
	${($.get(item).x += 2) ?? ""}
	${($.get(item).x -= 3) ?? ""}
	${$.get(item).x++ ?? ""}
	${$.get(item).x-- ?? ""}
	${++$.get(item).x ?? ""}
	${--$.get(item).x ?? ""}
	${($.get(item).x &&= 4) ?? ""}
	${($.get(item).x ||= 5) ?? ""}
	${($.get(item).x ??= 6) ?? ""}
	${($.get(item)["x"] = 1) ?? ""}
	${($.get(item)["x"] += 2) ?? ""}
	${$.get(item)["x"]++ ?? ""}
	${++$.get(item)["x"] ?? ""}
	${($.get(item)[key] = 1) ?? ""}
	${($.get(item)[key] += 2) ?? ""}
	${$.get(item)[key]++ ?? ""}
	${++$.get(item)[key] ?? ""} `));
		$.delegated("click", button_1, function click_1() {
			$.get(item).x = 1;
			$.get(item).x += 2;
			$.get(item).x++;
			++$.get(item).x;
			$.get(item).x &&= 4;
			$.get(item).x ||= 5;
			$.get(item).x ??= 6;
			$.get(item)["x"] = 1;
			$.get(item)[key] = 1;
			$.get(item)[key]++;
		});
		$.append($$anchor, fragment_2);
	}), "each", App, 332, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.each(node_1, 17, () => items, (it) => it.id, ($$anchor, it) => {
		const ctx = $.tag($.derived(() => $.get(it)), "ctx");
		$.get(ctx);
		$.next();
		var text_3 = $.text();
		$.template_effect(() => $.set_text(text_3, `${$.get(ctx).x ?? ""}
	${($.get(ctx).x = 1) ?? ""}
	${($.get(ctx).x += 2) ?? ""}
	${$.get(ctx).x++ ?? ""}
	${++$.get(ctx).x ?? ""}
	${($.get(ctx).x &&= 3) ?? ""}
	${($.get(ctx).x ||= 4) ?? ""}
	${($.get(ctx).x ??= 5) ?? ""}
	${($.get(ctx)["x"] = 1) ?? ""}
	${($.get(ctx)[key] = 1) ?? ""}`));
		$.append($$anchor, text_3);
	}), "each", App, 395, 0);
	var node_2 = $.sibling(node_1, 2);
	$.add_svelte_meta(() => $.await(node_2, () => promise, null, ($$anchor, v) => {
		var text_4 = $.text();
		$.template_effect(() => $.set_text(text_4, `${$.get(v).x ?? ""}
	${($.get(v).x = 1) ?? ""}
	${($.get(v).x += 2) ?? ""}
	${$.get(v).x++ ?? ""}
	${++$.get(v).x ?? ""}
	${($.get(v).x &&= 3) ?? ""}
	${($.get(v).x ||= 4) ?? ""}
	${($.get(v).x ??= 5) ?? ""}
	${($.get(v)["x"] = 1) ?? ""}
	${($.get(v)[key] = 1) ?? ""}`));
		$.append($$anchor, text_4);
	}, ($$anchor, e) => {
		var text_5 = $.text();
		$.template_effect(() => $.set_text(text_5, `${$.get(e).message ?? ""}
	${($.get(e).x = 1) ?? ""}
	${($.get(e).x += 2) ?? ""}
	${$.get(e).x++ ?? ""}
	${++$.get(e).x ?? ""}
	${($.get(e)["x"] = 1) ?? ""}
	${($.get(e)[key] = 1) ?? ""}`));
		$.append($$anchor, text_5);
	}), "await", App, 409, 0);
	var button_2 = $.sibling(node_2, 2);
	$.template_effect(() => $.set_text(text_1, `${$.get(count) ?? ""}
${countObj.x ?? ""}
${$.get(raw) ?? ""}
${deep.a.b.c.x ?? ""}
${deep["a"]?.["b"]?.["c"]?.["x"] ?? ""}
${deep?.a?.b?.c?.x ?? ""}
${deep[nestedKey]?.c?.[key] ?? ""}
${prop() ?? ""}
${propObj().x ?? ""}
${bind() ?? ""}
${bindObj().x ?? ""}
${$store() ?? ""}
${$objStore().x ?? ""}

${$.set(count, 1) ?? ""}
${$.set(count, $.get(count) + 2) ?? ""}
${$.set(count, $.get(count) - 3) ?? ""}
${$.update(count) ?? ""}
${$.update(count, -1) ?? ""}
${$.update_pre(count) ?? ""}
${$.update_pre(count, -1) ?? ""}
${$.set(count, $.get(count) && 4) ?? ""}
${$.set(count, $.get(count) || 5) ?? ""}
${$.set(count, $.get(count) ?? 6) ?? ""}

${(countObj.x = 7) ?? ""}
${(countObj.x += 8) ?? ""}
${countObj.x++ ?? ""}
${countObj.x-- ?? ""}
${++countObj.x ?? ""}
${--countObj.x ?? ""}
${(countObj.x &&= 9) ?? ""}
${(countObj.x ||= 10) ?? ""}
${(countObj.x ??= 11) ?? ""}
${(countObj["x"] = 7) ?? ""}
${(countObj["x"] += 8) ?? ""}
${countObj["x"]++ ?? ""}
${++countObj["x"] ?? ""}
${(countObj[key] = 7) ?? ""}
${(countObj[key] += 8) ?? ""}
${countObj[key]++ ?? ""}
${++countObj[key] ?? ""}

${(deep.a.b.c.x = 1) ?? ""}
${(deep.a.b.c.x += 2) ?? ""}
${(deep.a.b.c.x -= 3) ?? ""}
${deep.a.b.c.x++ ?? ""}
${deep.a.b.c.x-- ?? ""}
${++deep.a.b.c.x ?? ""}
${--deep.a.b.c.x ?? ""}
${(deep.a.b.c.x &&= 4) ?? ""}
${(deep.a.b.c.x ||= 5) ?? ""}
${(deep.a.b.c.x ??= 6) ?? ""}
${(deep["a"]["b"]["c"]["x"] = 1) ?? ""}
${(deep["a"]["b"]["c"]["x"] += 2) ?? ""}
${deep["a"]["b"]["c"]["x"]++ ?? ""}
${++deep["a"]["b"]["c"]["x"] ?? ""}
${(deep[nestedKey].c[key] = 1) ?? ""}
${(deep[nestedKey].c[key] += 2) ?? ""}
${deep[nestedKey].c[key]++ ?? ""}
${++deep[nestedKey].c[key] ?? ""}
${(deep.a[nestedKey].c.x = 1) ?? ""}
${(deep.a[nestedKey].c.x += 2) ?? ""}
${deep.a[nestedKey].c.x++ ?? ""}
${(deep.a.b.c[key] = 1) ?? ""}
${deep.a.b.c[key]++ ?? ""}

${$.set(raw, 12) ?? ""}
${$.set(raw, $.get(raw) + 13) ?? ""}
${$.set(raw, $.get(raw) - 14) ?? ""}
${$.update(raw) ?? ""}
${$.update(raw, -1) ?? ""}
${$.update_pre(raw) ?? ""}
${$.update_pre(raw, -1) ?? ""}
${$.set(raw, $.get(raw) && 15) ?? ""}
${$.set(raw, $.get(raw) || 16) ?? ""}
${$.set(raw, $.get(raw) ?? 17) ?? ""}

${prop(18) ?? ""}
${prop(prop() + 19) ?? ""}
${$.update_prop(prop) ?? ""}
${$.update_prop(prop, -1) ?? ""}
${$.update_pre_prop(prop) ?? ""}
${$.update_pre_prop(prop, -1) ?? ""}
${prop(prop() && 20) ?? ""}
${prop(prop() || 21) ?? ""}
${prop(prop() ?? 22) ?? ""}

${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x = 23, 258, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x += 24, 259, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x++, 260, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x--, 261, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], ++propObj().x, 262, 3) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], --propObj().x, 263, 3) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x &&= 25, 264, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x ||= 26, 265, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj().x ??= 27, 266, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj()["x"] = 23, 267, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj()["x"] += 24, 268, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], propObj()["x"]++, 269, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", "x"], ++propObj()["x"], 270, 3) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", key], propObj()[key] = 23, 271, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", key], propObj()[key] += 24, 272, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", key], propObj()[key]++, 273, 1) ?? ""}
${$$ownership_validator.mutation("propObj", ["propObj", key], ++propObj()[key], 274, 3) ?? ""}

${bind(28) ?? ""}
${bind(bind() + 29) ?? ""}
${$.update_prop(bind) ?? ""}
${$.update_prop(bind, -1) ?? ""}
${$.update_pre_prop(bind) ?? ""}
${$.update_pre_prop(bind, -1) ?? ""}
${bind(bind() && 30) ?? ""}
${bind(bind() || 31) ?? ""}
${bind(bind() ?? 32) ?? ""}

${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x = 33, true), 286, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x += 34, true), 287, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x++, true), 288, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x--, true), 289, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(++bindObj().x, true), 290, 3) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(--bindObj().x, true), 291, 3) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x &&= 35, true), 292, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x ||= 36, true), 293, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj().x ??= 37, true), 294, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj()["x"] = 33, true), 295, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj()["x"] += 34, true), 296, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(bindObj()["x"]++, true), 297, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", "x"], bindObj(++bindObj()["x"], true), 298, 3) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(bindObj()[key] = 33, true), 299, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(bindObj()[key] += 34, true), 300, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(bindObj()[key]++, true), 301, 1) ?? ""}
${$$ownership_validator.mutation("bindObj", ["bindObj", key], bindObj(++bindObj()[key], true), 302, 3) ?? ""}

${$.store_set(store, 38) ?? ""}
${$.store_set(store, $store() + 39) ?? ""}
${$.update_store(store, $store()) ?? ""}
${$.update_store(store, $store(), -1) ?? ""}
${$.update_pre_store(store, $store()) ?? ""}
${$.update_pre_store(store, $store(), -1) ?? ""}
${$.store_set(store, $store() && 40) ?? ""}
${$.store_set(store, $store() || 41) ?? ""}
${$.store_set(store, $store() ?? 42) ?? ""}

${$.store_mutate(objStore, $.untrack($objStore).x = 43, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore).x += 44, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore).x &&= 45, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore).x ||= 46, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore).x ??= 47, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore)["x"] = 43, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore)["x"] += 44, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore)[key] = 43, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore)[key] += 44, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore)) ?? ""}
${$.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore)) ?? ""} `));
	$.delegated("click", button_2, script_ops);
	$.append($$anchor, fragment_1);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
