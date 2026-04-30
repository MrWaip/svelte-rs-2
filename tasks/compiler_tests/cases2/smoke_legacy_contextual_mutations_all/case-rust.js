import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root_1 = $.from_html(` <button>snippet</button>`, 1);
var root_2 = $.from_html(` <button>each-row</button>`, 1);
var root = $.from_html(` <!> <!> <!> <button>run</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $store = () => $.store_get(store, "$store", $$stores);
	const $objStore = () => $.store_get(objStore, "$objStore", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const card = ($$anchor, param = $.noop) => {
		$.next();
		var fragment = root_1();
		var text = $.first_child(fragment);
		var button = $.sibling(text);
		$.template_effect(() => $.set_text(text, `${param() ?? ""}
	${(param(), $.untrack(() => param().x)) ?? ""}
	${(param(), $.untrack(() => param().x = 1)) ?? ""}
	${(param(), $.untrack(() => param().x += 2)) ?? ""}
	${(param(), $.untrack(() => param().x++)) ?? ""}
	${(param(), $.untrack(() => ++param().x)) ?? ""}
	${(param(), $.untrack(() => param().x &&= 3)) ?? ""}
	${(param(), $.untrack(() => param().x ||= 4)) ?? ""}
	${(param(), $.untrack(() => param().x ??= 5)) ?? ""}
	${(param(), $.untrack(() => param()["x"] = 1)) ?? ""}
	${(param(), $.untrack(() => param()["x"]++)) ?? ""}
	${(param(), $.untrack(() => param()[key] = 1)) ?? ""}
	${(param(), $.untrack(() => param()[key]++)) ?? ""} `));
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
	let count = $.prop($$props, "count", 12, 0);
	let obj = $.prop($$props, "obj", 28, () => ({ x: 0 }));
	let local = $.mutable_source(0);
	let localObj = $.mutable_source({ x: 0 });
	let key = "x";
	const store = writable(0);
	const objStore = writable({ x: 0 });
	let items = [{
		id: 1,
		x: 0
	}, {
		id: 2,
		x: 0
	}];
	let promise = Promise.resolve({ x: 0 });
	function script_ops() {
		count(1);
		count(count() + 2);
		count(count() - 3);
		$.update_prop(count);
		$.update_prop(count, -1);
		$.update_pre_prop(count);
		$.update_pre_prop(count, -1);
		count(count() && 4);
		count(count() || 5);
		count(count() ?? 6);
		obj(obj().x = 7, true);
		obj(obj().x += 8, true);
		obj(obj().x++, true);
		obj(obj().x--, true);
		obj(++obj().x, true);
		obj(--obj().x, true);
		obj(obj().x &&= 9, true);
		obj(obj().x ||= 10, true);
		obj(obj().x ??= 11, true);
		obj(obj()["x"] = 7, true);
		obj(obj()["x"] += 8, true);
		obj(obj()["x"]++, true);
		obj(++obj()["x"], true);
		obj(obj()[key] = 7, true);
		obj(obj()[key] += 8, true);
		obj(obj()[key]++, true);
		obj(++obj()[key], true);
		$.set(local, 12);
		$.set(local, $.get(local) + 13);
		$.set(local, $.get(local) - 14);
		$.update(local);
		$.update(local, -1);
		$.update_pre(local);
		$.update_pre(local, -1);
		$.set(local, $.get(local) && 15);
		$.set(local, $.get(local) || 16);
		$.set(local, $.get(local) ?? 17);
		$.mutate(localObj, $.get(localObj).x = 18);
		$.mutate(localObj, $.get(localObj).x += 19);
		$.mutate(localObj, $.get(localObj).x++);
		$.mutate(localObj, $.get(localObj).x--);
		$.mutate(localObj, ++$.get(localObj).x);
		$.mutate(localObj, --$.get(localObj).x);
		$.mutate(localObj, $.get(localObj).x &&= 20);
		$.mutate(localObj, $.get(localObj).x ||= 21);
		$.mutate(localObj, $.get(localObj).x ??= 22);
		$.mutate(localObj, $.get(localObj)["x"] = 18);
		$.mutate(localObj, $.get(localObj)["x"] += 19);
		$.mutate(localObj, $.get(localObj)["x"]++);
		$.mutate(localObj, ++$.get(localObj)["x"]);
		$.mutate(localObj, $.get(localObj)[key] = 18);
		$.mutate(localObj, $.get(localObj)[key] += 19);
		$.mutate(localObj, $.get(localObj)[key]++);
		$.mutate(localObj, ++$.get(localObj)[key]);
		$.store_set(store, 23);
		$.store_set(store, $store() + 24);
		$.store_set(store, $store() - 25);
		$.update_store(store, $store());
		$.update_store(store, $store(), -1);
		$.update_pre_store(store, $store());
		$.update_pre_store(store, $store(), -1);
		$.store_set(store, $store() && 26);
		$.store_set(store, $store() || 27);
		$.store_set(store, $store() ?? 28);
		$.store_mutate(objStore, $.untrack($objStore).x = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x &&= 31, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ||= 32, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ??= 33, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore));
	}
	$.init();
	$.next();
	var fragment_1 = root();
	var text_1 = $.first_child(fragment_1);
	var node = $.sibling(text_1);
	$.each(node, 3, () => items, (item) => item.id, ($$anchor, item, i) => {
		$.next();
		var fragment_2 = root_2();
		var text_2 = $.first_child(fragment_2);
		var button_1 = $.sibling(text_2);
		$.template_effect(() => $.set_text(text_2, `${$.get(item) ?? ""}
	${$.get(i) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x = 1)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x += 2)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x -= 3)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x++)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x--)) ?? ""}
	${($.get(item), $.untrack(() => ++$.get(item).x)) ?? ""}
	${($.get(item), $.untrack(() => --$.get(item).x)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x &&= 4)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x ||= 5)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x ??= 6)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item)["x"] = 1)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item)["x"] += 2)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item)["x"]++)) ?? ""}
	${($.get(item), $.untrack(() => ++$.get(item)["x"])) ?? ""}
	${($.get(item), $.untrack(() => $.get(item)[key] = 1)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item)[key] += 2)) ?? ""}
	${($.get(item), $.untrack(() => $.get(item)[key]++)) ?? ""}
	${($.get(item), $.untrack(() => ++$.get(item)[key])) ?? ""} `));
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
	$.each(node_1, 1, () => items, (it) => it.id, ($$anchor, it) => {
		const ctx = $.derived(() => $.get(it));
		$.next();
		var text_3 = $.text();
		$.template_effect(() => $.set_text(text_3, `${($.get(ctx), $.untrack(() => $.get(ctx).x)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx).x = 1)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx).x += 2)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx).x++)) ?? ""}
	${($.get(ctx), $.untrack(() => ++$.get(ctx).x)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx).x &&= 3)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx).x ||= 4)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx).x ??= 5)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx)["x"] = 1)) ?? ""}
	${($.get(ctx), $.untrack(() => $.get(ctx)[key] = 1)) ?? ""}`));
		$.append($$anchor, text_3);
	});
	var node_2 = $.sibling(node_1, 2);
	$.await(node_2, () => promise, null, ($$anchor, v) => {
		var text_4 = $.text();
		$.template_effect(() => $.set_text(text_4, `${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x = 1)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x += 2)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x++)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => ++$.get(v).x)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x &&= 3)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x ||= 4)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x ??= 5)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v)["x"] = 1)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v)[key] = 1)) ?? ""}`));
		$.append($$anchor, text_4);
	}, ($$anchor, e) => {
		var text_5 = $.text();
		$.template_effect(() => $.set_text(text_5, `${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).message)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).x = 1)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).x += 2)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).x++)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => ++$.get(e).x)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e)["x"] = 1)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e)[key] = 1)) ?? ""}`));
		$.append($$anchor, text_5);
	});
	var button_2 = $.sibling(node_2, 2);
	$.template_effect(() => $.set_text(text_1, `${count() ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj().x)) ?? ""}
${$.get(local) ?? ""}
${($.get(localObj), $.untrack(() => $.get(localObj).x)) ?? ""}
${$store() ?? ""}
${($objStore(), $.untrack(() => $objStore().x)) ?? ""}

${($.deep_read_state(count()), $.untrack(() => count(1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() + 2))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() - 3))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_prop(count))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_prop(count, -1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_pre_prop(count))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_pre_prop(count, -1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() && 4))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() || 5))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() ?? 6))) ?? ""}

${($.deep_read_state(obj()), $.untrack(() => obj(obj().x = 7, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x += 8, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x++, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x--, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(++obj().x, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(--obj().x, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x &&= 9, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x ||= 10, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x ??= 11, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()["x"] = 7, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()["x"] += 8, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()["x"]++, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(++obj()["x"], true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()[key] = 7, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()[key] += 8, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()[key]++, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(++obj()[key], true))) ?? ""}

${($.get(local), $.untrack(() => $.set(local, 12))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) + 13))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) - 14))) ?? ""}
${($.get(local), $.untrack(() => $.update(local))) ?? ""}
${($.get(local), $.untrack(() => $.update(local, -1))) ?? ""}
${($.get(local), $.untrack(() => $.update_pre(local))) ?? ""}
${($.get(local), $.untrack(() => $.update_pre(local, -1))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) && 15))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) || 16))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) ?? 17))) ?? ""}

${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x--))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj).x))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, --$.get(localObj).x))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x &&= 20))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x ||= 21))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x ??= 22))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"] = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"] += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"]++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj)["x"]))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key] = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key] += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key]++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj)[key]))) ?? ""}

${($store(), $.untrack(() => $.store_set(store, 23))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() + 24))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() - 25))) ?? ""}
${($store(), $.untrack(() => $.update_store(store, $store()))) ?? ""}
${($store(), $.untrack(() => $.update_store(store, $store(), -1))) ?? ""}
${($store(), $.untrack(() => $.update_pre_store(store, $store()))) ?? ""}
${($store(), $.untrack(() => $.update_pre_store(store, $store(), -1))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() && 26))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() || 27))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() ?? 28))) ?? ""}

${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x &&= 31, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x ||= 32, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x ??= 33, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"] = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"] += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key] = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key] += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore)))) ?? ""} `));
	$.delegated("click", button_2, script_ops);
	$.append($$anchor, fragment_1);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
