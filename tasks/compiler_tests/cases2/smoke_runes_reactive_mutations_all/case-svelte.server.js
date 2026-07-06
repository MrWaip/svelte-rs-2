import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { prop = 0, propObj = { x: 0 }, bind = 0, bindObj = { x: 0 } } = $$props;
		let count = 0;
		let countObj = { x: 0 };
		let raw = 0;
		let deep = { a: { b: { c: { x: 0 } } } };
		let key = "x";
		let nestedKey = "b";
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
			count = 1;
			count += 2;
			count -= 3;
			count++;
			count--;
			++count;
			--count;
			count &&= 4;
			count ||= 5;
			count ??= 6;
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
			raw = 12;
			raw += 13;
			raw -= 14;
			raw++;
			raw--;
			++raw;
			--raw;
			raw &&= 15;
			raw ||= 16;
			raw ??= 17;
			prop = 18;
			prop += 19;
			prop++;
			prop--;
			++prop;
			--prop;
			prop &&= 20;
			prop ||= 21;
			prop ??= 22;
			propObj.x = 23;
			propObj.x += 24;
			propObj.x++;
			propObj.x--;
			++propObj.x;
			--propObj.x;
			propObj.x &&= 25;
			propObj.x ||= 26;
			propObj.x ??= 27;
			propObj["x"] = 23;
			propObj["x"] += 24;
			propObj["x"]++;
			++propObj["x"];
			propObj[key] = 23;
			propObj[key] += 24;
			propObj[key]++;
			++propObj[key];
			bind = 28;
			bind += 29;
			bind++;
			bind--;
			++bind;
			--bind;
			bind &&= 30;
			bind ||= 31;
			bind ??= 32;
			bindObj.x = 33;
			bindObj.x += 34;
			bindObj.x++;
			bindObj.x--;
			++bindObj.x;
			--bindObj.x;
			bindObj.x &&= 35;
			bindObj.x ||= 36;
			bindObj.x ??= 37;
			bindObj["x"] = 33;
			bindObj["x"] += 34;
			bindObj["x"]++;
			++bindObj["x"];
			bindObj[key] = 33;
			bindObj[key] += 34;
			bindObj[key]++;
			++bindObj[key];
			$.store_set(store, 38);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) + 39);
			$.update_store($$store_subs ??= {}, "$store", store);
			$.update_store($$store_subs ??= {}, "$store", store, -1);
			$.update_store_pre($$store_subs ??= {}, "$store", store);
			$.update_store_pre($$store_subs ??= {}, "$store", store, -1);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) && 40);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) || 41);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) ?? 42);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x = 43);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x += 44);
			$.store_get($$store_subs ??= {}, "$objStore", objStore).x++;
			$.store_get($$store_subs ??= {}, "$objStore", objStore).x--;
			++$.store_get($$store_subs ??= {}, "$objStore", objStore).x;
			--$.store_get($$store_subs ??= {}, "$objStore", objStore).x;
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x &&= 45);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ||= 46);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ??= 47);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] = 43);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] += 44);
			$.store_get($$store_subs ??= {}, "$objStore", objStore)["x"]++;
			++$.store_get($$store_subs ??= {}, "$objStore", objStore)["x"];
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] = 43);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] += 44);
			$.store_get($$store_subs ??= {}, "$objStore", objStore)[key]++;
			++$.store_get($$store_subs ??= {}, "$objStore", objStore)[key];
		}
		function card($$renderer, param) {
			$$renderer.push(`<!---->${$.escape(param)}
	${$.escape(param.x)}
	${$.escape(param.x = 1)}
	${$.escape(param.x += 2)}
	${$.escape(param.x++)}
	${$.escape(++param.x)}
	${$.escape(param.x &&= 3)}
	${$.escape(param.x ||= 4)}
	${$.escape(param.x ??= 5)}
	${$.escape(param["x"] = 1)}
	${$.escape(param["x"]++)}
	${$.escape(param[key] = 1)}
	${$.escape(param[key]++)} <button>snippet</button>`);
		}
		$$renderer.push(`<!---->${$.escape(count)}
${$.escape(countObj.x)}
${$.escape(raw)}
${$.escape(deep.a.b.c.x)}
${$.escape(deep["a"]?.["b"]?.["c"]?.["x"])}
${$.escape(deep?.a?.b?.c?.x)}
${$.escape(deep[nestedKey]?.c?.[key])}
${$.escape(prop)}
${$.escape(propObj.x)}
${$.escape(bind)}
${$.escape(bindObj.x)}
${$.escape($.store_get($$store_subs ??= {}, "$store", store))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore).x)}

${$.escape(count = 1)}
${$.escape(count += 2)}
${$.escape(count -= 3)}
${$.escape(count++)}
${$.escape(count--)}
${$.escape(++count)}
${$.escape(--count)}
${$.escape(count &&= 4)}
${$.escape(count ||= 5)}
${$.escape(count ??= 6)}

${$.escape(countObj.x = 7)}
${$.escape(countObj.x += 8)}
${$.escape(countObj.x++)}
${$.escape(countObj.x--)}
${$.escape(++countObj.x)}
${$.escape(--countObj.x)}
${$.escape(countObj.x &&= 9)}
${$.escape(countObj.x ||= 10)}
${$.escape(countObj.x ??= 11)}
${$.escape(countObj["x"] = 7)}
${$.escape(countObj["x"] += 8)}
${$.escape(countObj["x"]++)}
${$.escape(++countObj["x"])}
${$.escape(countObj[key] = 7)}
${$.escape(countObj[key] += 8)}
${$.escape(countObj[key]++)}
${$.escape(++countObj[key])}

${$.escape(deep.a.b.c.x = 1)}
${$.escape(deep.a.b.c.x += 2)}
${$.escape(deep.a.b.c.x -= 3)}
${$.escape(deep.a.b.c.x++)}
${$.escape(deep.a.b.c.x--)}
${$.escape(++deep.a.b.c.x)}
${$.escape(--deep.a.b.c.x)}
${$.escape(deep.a.b.c.x &&= 4)}
${$.escape(deep.a.b.c.x ||= 5)}
${$.escape(deep.a.b.c.x ??= 6)}
${$.escape(deep["a"]["b"]["c"]["x"] = 1)}
${$.escape(deep["a"]["b"]["c"]["x"] += 2)}
${$.escape(deep["a"]["b"]["c"]["x"]++)}
${$.escape(++deep["a"]["b"]["c"]["x"])}
${$.escape(deep[nestedKey].c[key] = 1)}
${$.escape(deep[nestedKey].c[key] += 2)}
${$.escape(deep[nestedKey].c[key]++)}
${$.escape(++deep[nestedKey].c[key])}
${$.escape(deep.a[nestedKey].c.x = 1)}
${$.escape(deep.a[nestedKey].c.x += 2)}
${$.escape(deep.a[nestedKey].c.x++)}
${$.escape(deep.a.b.c[key] = 1)}
${$.escape(deep.a.b.c[key]++)}

${$.escape(raw = 12)}
${$.escape(raw += 13)}
${$.escape(raw -= 14)}
${$.escape(raw++)}
${$.escape(raw--)}
${$.escape(++raw)}
${$.escape(--raw)}
${$.escape(raw &&= 15)}
${$.escape(raw ||= 16)}
${$.escape(raw ??= 17)}

${$.escape(prop = 18)}
${$.escape(prop += 19)}
${$.escape(prop++)}
${$.escape(prop--)}
${$.escape(++prop)}
${$.escape(--prop)}
${$.escape(prop &&= 20)}
${$.escape(prop ||= 21)}
${$.escape(prop ??= 22)}

${$.escape(propObj.x = 23)}
${$.escape(propObj.x += 24)}
${$.escape(propObj.x++)}
${$.escape(propObj.x--)}
${$.escape(++propObj.x)}
${$.escape(--propObj.x)}
${$.escape(propObj.x &&= 25)}
${$.escape(propObj.x ||= 26)}
${$.escape(propObj.x ??= 27)}
${$.escape(propObj["x"] = 23)}
${$.escape(propObj["x"] += 24)}
${$.escape(propObj["x"]++)}
${$.escape(++propObj["x"])}
${$.escape(propObj[key] = 23)}
${$.escape(propObj[key] += 24)}
${$.escape(propObj[key]++)}
${$.escape(++propObj[key])}

${$.escape(bind = 28)}
${$.escape(bind += 29)}
${$.escape(bind++)}
${$.escape(bind--)}
${$.escape(++bind)}
${$.escape(--bind)}
${$.escape(bind &&= 30)}
${$.escape(bind ||= 31)}
${$.escape(bind ??= 32)}

${$.escape(bindObj.x = 33)}
${$.escape(bindObj.x += 34)}
${$.escape(bindObj.x++)}
${$.escape(bindObj.x--)}
${$.escape(++bindObj.x)}
${$.escape(--bindObj.x)}
${$.escape(bindObj.x &&= 35)}
${$.escape(bindObj.x ||= 36)}
${$.escape(bindObj.x ??= 37)}
${$.escape(bindObj["x"] = 33)}
${$.escape(bindObj["x"] += 34)}
${$.escape(bindObj["x"]++)}
${$.escape(++bindObj["x"])}
${$.escape(bindObj[key] = 33)}
${$.escape(bindObj[key] += 34)}
${$.escape(bindObj[key]++)}
${$.escape(++bindObj[key])}

${$.escape($.store_set(store, 38))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) + 39))}
${$.escape($.update_store($$store_subs ??= {}, "$store", store))}
${$.escape($.update_store($$store_subs ??= {}, "$store", store, -1))}
${$.escape($.update_store_pre($$store_subs ??= {}, "$store", store))}
${$.escape($.update_store_pre($$store_subs ??= {}, "$store", store, -1))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) && 40))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) || 41))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) ?? 42))}

${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x = 43))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x += 44))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore).x++)}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore).x--)}
${$.escape(++$.store_get($$store_subs ??= {}, "$objStore", objStore).x)}
${$.escape(--$.store_get($$store_subs ??= {}, "$objStore", objStore).x)}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x &&= 45))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ||= 46))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ??= 47))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] = 43))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] += 44))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore)["x"]++)}
${$.escape(++$.store_get($$store_subs ??= {}, "$objStore", objStore)["x"])}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] = 43))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] += 44))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore)[key]++)}
${$.escape(++$.store_get($$store_subs ??= {}, "$objStore", objStore)[key])} <!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let item = each_array[i];
			$$renderer.push(`<!---->${$.escape(item)}
	${$.escape(i)}
	${$.escape(item.x)}
	${$.escape(item.x = 1)}
	${$.escape(item.x += 2)}
	${$.escape(item.x -= 3)}
	${$.escape(item.x++)}
	${$.escape(item.x--)}
	${$.escape(++item.x)}
	${$.escape(--item.x)}
	${$.escape(item.x &&= 4)}
	${$.escape(item.x ||= 5)}
	${$.escape(item.x ??= 6)}
	${$.escape(item["x"] = 1)}
	${$.escape(item["x"] += 2)}
	${$.escape(item["x"]++)}
	${$.escape(++item["x"])}
	${$.escape(item[key] = 1)}
	${$.escape(item[key] += 2)}
	${$.escape(item[key]++)}
	${$.escape(++item[key])} <button>each-row</button>`);
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let it = each_array_1[$$index_1];
			const ctx = it;
			$$renderer.push(`<!---->${$.escape(ctx.x)}
	${$.escape(ctx.x = 1)}
	${$.escape(ctx.x += 2)}
	${$.escape(ctx.x++)}
	${$.escape(++ctx.x)}
	${$.escape(ctx.x &&= 3)}
	${$.escape(ctx.x ||= 4)}
	${$.escape(ctx.x ??= 5)}
	${$.escape(ctx["x"] = 1)}
	${$.escape(ctx[key] = 1)}`);
		}
		$$renderer.push(`<!--]--> `);
		$.await($$renderer, promise, () => {}, (v) => {
			$$renderer.push(`${$.escape(v.x)}
	${$.escape(v.x = 1)}
	${$.escape(v.x += 2)}
	${$.escape(v.x++)}
	${$.escape(++v.x)}
	${$.escape(v.x &&= 3)}
	${$.escape(v.x ||= 4)}
	${$.escape(v.x ??= 5)}
	${$.escape(v["x"] = 1)}
	${$.escape(v[key] = 1)}`);
		});
		$$renderer.push(`<!--]--> <button>run</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			bind,
			bindObj
		});
	});
}
