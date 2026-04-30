import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root_1 = $.from_html(` <button>snippet</button>`, 1);
var root_2 = $.from_html(` <button>each-row</button>`, 1);
var root = $.from_html(` <!> <!> <!> <button>run</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $store = () => $.store_get(store, "$store", $$stores);
	const $objStore = () => $.store_get(objStore, "$objStore", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const card = ($$anchor, param = $.noop) => {
		$.next();
		var fragment = root_1();
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
		$.delegated("click", button, () => {
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
	};
	let prop = $.prop($$props, "prop", 7, 0), propObj = $.prop($$props, "propObj", 23, () => ({ x: 0 })), bind = $.prop($$props, "bind", 15, 0), bindObj = $.prop($$props, "bindObj", 31, () => $.proxy({ x: 0 }));
	let count = $.state(0);
	let countObj = $.proxy({ x: 0 });
	let raw = $.state(0);
	let deep = $.proxy({ a: { b: { c: { x: 0 } } } });
	let key = "x";
	let nestedKey = "b";
	const store = writable(0);
	const objStore = writable({ x: 0 });
	let items = $.proxy([{
		id: 1,
		x: 0
	}, {
		id: 2,
		x: 0
	}]);
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
		propObj().x = 23;
		propObj().x += 24;
		propObj().x++;
		propObj().x--;
		++propObj().x;
		--propObj().x;
		propObj().x &&= 25;
		propObj().x ||= 26;
		propObj().x ??= 27;
		propObj()["x"] = 23;
		propObj()["x"] += 24;
		propObj()["x"]++;
		++propObj()["x"];
		propObj()[key] = 23;
		propObj()[key] += 24;
		propObj()[key]++;
		++propObj()[key];
		bind(28);
		bind(bind() + 29);
		$.update_prop(bind);
		$.update_prop(bind, -1);
		$.update_pre_prop(bind);
		$.update_pre_prop(bind, -1);
		bind(bind() && 30);
		bind(bind() || 31);
		bind(bind() ?? 32);
		bindObj(bindObj().x = 33, true);
		bindObj(bindObj().x += 34, true);
		bindObj(bindObj().x++, true);
		bindObj(bindObj().x--, true);
		bindObj(++bindObj().x, true);
		bindObj(--bindObj().x, true);
		bindObj(bindObj().x &&= 35, true);
		bindObj(bindObj().x ||= 36, true);
		bindObj(bindObj().x ??= 37, true);
		bindObj(bindObj()["x"] = 33, true);
		bindObj(bindObj()["x"] += 34, true);
		bindObj(bindObj()["x"]++, true);
		bindObj(++bindObj()["x"], true);
		bindObj(bindObj()[key] = 33, true);
		bindObj(bindObj()[key] += 34, true);
		bindObj(bindObj()[key]++, true);
		bindObj(++bindObj()[key], true);
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
	$.next();
	var fragment_1 = root();
	var text_1 = $.first_child(fragment_1);
	var node = $.sibling(text_1);
	$.each(node, 19, () => items, (item) => item.id, ($$anchor, item, i) => {
		$.next();
		var fragment_2 = root_2();
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
		$.delegated("click", button_1, () => {
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
	});
	var node_1 = $.sibling(node, 2);
	$.each(node_1, 17, () => items, (it) => it.id, ($$anchor, it) => {
		const ctx = $.derived(() => $.get(it));
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
	});
	var node_2 = $.sibling(node_1, 2);
	$.await(node_2, () => promise, null, ($$anchor, v) => {
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
	});
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

${(propObj().x = 23) ?? ""}
${(propObj().x += 24) ?? ""}
${propObj().x++ ?? ""}
${propObj().x-- ?? ""}
${++propObj().x ?? ""}
${--propObj().x ?? ""}
${(propObj().x &&= 25) ?? ""}
${(propObj().x ||= 26) ?? ""}
${(propObj().x ??= 27) ?? ""}
${(propObj()["x"] = 23) ?? ""}
${(propObj()["x"] += 24) ?? ""}
${propObj()["x"]++ ?? ""}
${++propObj()["x"] ?? ""}
${(propObj()[key] = 23) ?? ""}
${(propObj()[key] += 24) ?? ""}
${propObj()[key]++ ?? ""}
${++propObj()[key] ?? ""}

${bind(28) ?? ""}
${bind(bind() + 29) ?? ""}
${$.update_prop(bind) ?? ""}
${$.update_prop(bind, -1) ?? ""}
${$.update_pre_prop(bind) ?? ""}
${$.update_pre_prop(bind, -1) ?? ""}
${bind(bind() && 30) ?? ""}
${bind(bind() || 31) ?? ""}
${bind(bind() ?? 32) ?? ""}

${bindObj(bindObj().x = 33, true) ?? ""}
${bindObj(bindObj().x += 34, true) ?? ""}
${bindObj(bindObj().x++, true) ?? ""}
${bindObj(bindObj().x--, true) ?? ""}
${bindObj(++bindObj().x, true) ?? ""}
${bindObj(--bindObj().x, true) ?? ""}
${bindObj(bindObj().x &&= 35, true) ?? ""}
${bindObj(bindObj().x ||= 36, true) ?? ""}
${bindObj(bindObj().x ??= 37, true) ?? ""}
${bindObj(bindObj()["x"] = 33, true) ?? ""}
${bindObj(bindObj()["x"] += 34, true) ?? ""}
${bindObj(bindObj()["x"]++, true) ?? ""}
${bindObj(++bindObj()["x"], true) ?? ""}
${bindObj(bindObj()[key] = 33, true) ?? ""}
${bindObj(bindObj()[key] += 34, true) ?? ""}
${bindObj(bindObj()[key]++, true) ?? ""}
${bindObj(++bindObj()[key], true) ?? ""}

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
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
