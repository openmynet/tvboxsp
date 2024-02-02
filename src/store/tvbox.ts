import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import localstorage from "./localstorage";
async function loadResource(uri: string) {
  uri = uri.trim();
  if (!uri) {
    return undefined;
  }
  const content = await invoke<undefined | TvBoxSource>("parse_tvbox", {
    uri,
  }).catch((e) => {
    console.log(e);
    return undefined;
  });
  content?.sites.forEach((item) => {
    item.features = features(item);
  });
  return content;
}
async function inlineCheckResource(items: any[], method: string) {
  if (!items.length) {
    return [];
  }
  console.time("check");
  const value = await invoke<ConnectionStatus[]>(method, {
    items,
  }).catch((e) => {
    console.log(e);
    return [];
  });
  console.timeEnd("check");
  return value;
}
async function checkVods(items: TvBoxVod[]) {
  return inlineCheckResource(items, "vods_connectivity");
}
async function checkLive(items: TvBoxLive[]) {
  return inlineCheckResource(items, "live_connectivity");
}
async function checkParses(items: TvBoxParse[]) {
  return inlineCheckResource(items, "parses_connectivity");
}

function features(item: TvBoxVod) {
  const feature = [];
  if (item.searchable) {
    feature.push("点播搜索");
  }
  if (item.searchable) {
    feature.push("快速搜索");
  }
  if (item.searchable) {
    feature.push("列表筛选");
  }
  return feature.join(", ");
}

const useTvBoxStore = defineStore("tvbox", () => {
  const SOURCE = "TVBOX_SOURCE";
  const MERGIN_SOURCE = "TVBOX_MERGIN_SOURCE";

  const source = ref(undefined as undefined | TvBoxSource);
  const source_text = computed(() => {
    return JSON.stringify(source.value, undefined, " ");
  });
  const merginSource = ref("");
  const loading = ref(false);
  const init = async () => {
    const ms = await localstorage.get<string>(MERGIN_SOURCE).catch((_) => "");
    if (ms) {
      merginSource.value = ms;
    }
    const src = await localstorage.get<string>(SOURCE).catch((_) => "");
    if (src) {
      try {
        source.value = JSON.parse(src);
      } catch (_) {}
    }
  };
  const done = async () => {
    await localstorage
      .set(SOURCE, JSON.stringify(source.value))
      .catch((_) => null);
    await localstorage.save().catch((_) => null);
    loading.value = false;
  };
  const load = async (uri: string) => {
    loading.value = true;
    const value = await loadResource(uri);
    source.value = value;
    await done();
  };
  const push = async (uri: string) => {
    loading.value = true;
    const value = await loadResource(uri);
    if (!source.value) {
      source.value = value;
      loading.value = false;
      return;
    }
    if (value) {
      value.sites.forEach((item) => {
        if (!item.jar) {
          item.jar = value.spider;
        }
      });
      // 合并点播
      const sites = source.value.sites.concat(value.sites);
      const sitesMap = {} as Record<string, TvBoxVod>;
      sites.forEach((item) => {
        const key = item.api + item.ext || "";
        if (!sitesMap[key]) {
          sitesMap[key] = item;
        }
      });
      source.value.sites = Object.values(sitesMap);
      // 合并直播
      const lives = source.value.lives.concat(value.lives);
      const livesMap = {} as Record<string, TvBoxLive>;
      lives.forEach((item) => {
        const key =
          item.url +
          (item.channels || []).map((chn) => chn.urls.join(",")).join(",");
        if (!livesMap[key]) {
          livesMap[key] = item;
        }
      });
      source.value.lives = Object.values(livesMap);
      // 合并解析器
      const parses = source.value.parses
        ? source.value.parses.concat(value.parses || [])
        : value.parses || [];
      const parsesMap = {} as Record<string, TvBoxParse>;
      parses.forEach((item) => {
        const key = item.url;
        if (!parsesMap[key]) {
          parsesMap[key] = item;
        }
      });
      source.value.parses = Object.values(parsesMap);
      // 合并vip标识
      if (value.flags) {
        if (source.value.flags) {
          const items = source.value.flags.concat(value.flags);
          source.value.flags = Array.from(new Set(items));
        } else {
          source.value.flags = value.flags;
        }
      }
      // 合并广告过滤规则
      if (value.ads) {
        if (source.value.ads) {
          const items = source.value.ads.concat(value.ads);
          source.value.ads = Array.from(new Set(items));
        } else {
          source.value.ads = value.ads;
        }
      }
      // 合并提取规则
      if (value.rules) {
        if (source.value.rules) {
          const items = source.value.rules.concat(value.rules);
          const rulesMap = {} as Record<string, TvBoxRule>;
          items.forEach((item) => {
            const key =
              item.host ||
              "" +
                (item.hosts || []).join(",") +
                (item.regex || []).join(",") +
                (item.rule || []).join(",");
            if (!rulesMap[key]) {
              rulesMap[key] = item;
            }
          });
          source.value.rules = Object.values(rulesMap);
        } else {
          source.value.rules = value.rules;
        }
      }
    }
    await done();
  };
  const check = async () => {
    if (!source.value) {
      return;
    }
    loading.value = true;
    const items = await checkVods(source.value.sites);
    const onlines = items.map((i) => {
      const item = i.extra as TvBoxVod;
      item.status = i.connectable ? 1 : -1;
      return item;
    });
    source.value.sites = onlines;
    await done();
  };
  const check_live = async (value?: TvBoxLive[]) => {
    if (!source.value) {
      return;
    }
    loading.value = true;
    const items = await checkLive(value || source.value.lives);
    const onlines = items.map((i) => {
      const item = i.extra as TvBoxLive;
      item.status = i.connectable ? 1 : -1;
      return item;
    });
    source.value.lives = onlines;
    await done();
  };
  const check_parses = async (value?: TvBoxParse[]) => {
    if (!source.value) {
      return;
    }
    loading.value = true;
    const items = await checkParses(value || source.value.parses || []);
    const onlines = items.map((i) => {
      const item = i.extra as TvBoxParse;
      item.status = i.connectable ? 1 : -1;
      return item;
    });
    source.value.parses = onlines;
    await done();
  };
  const check_by = async (list: TvBoxVod[]) => {
    if (!source.value) {
      return;
    }
    loading.value = true;
    const items = await checkVods(list);
    const onlines = {} as Record<string, number>;
    items.forEach((i) => {
      const item = i.extra as TvBoxVod;
      onlines[item.key] = i.connectable ? 1 : -1;
    });
    source.value.sites.forEach((i) => {
      if (onlines[i.key]) {
        i.status = onlines[i.key];
      }
    });
    await done();
  };
  const remove_by = async (items: TvBoxVod[]) => {
    if (!source.value) {
      return;
    }
    const keys = items.map((item) => item.name + "-" + item.key);
    source.value.sites = source.value?.sites.filter((item) => {
      const key = item.name + "-" + item.key;
      return !keys.includes(key);
    });
    await done();
  };
  const remove_live = async (i: number) => {
    if (!source.value) {
      return;
    }
    source.value.lives.splice(i, 1);
    await done();
  };
  const remove_parses = async (i: number | TvBoxParse[]) => {
    if (!source.value) {
      return;
    }
    if (typeof i == "number") {
      source.value.parses?.splice(i, 1);
    } else if (Array.isArray(i)) {
      const keys = i.map((item) => item.name + "-" + item.url);
      source.value.parses = source.value?.parses?.filter((item) => {
        const key = item.name + "-" + item.url;
        return !keys.includes(key);
      });
    }
    await done();
  };
  const remove_ads = async (i: number) => {
    if (!source.value || !source.value.ads) {
      return;
    }
    source.value.ads.splice(i, 1);
    await done();
  };
  const add_live = async (item: TvBoxLive) => {
    if (!source.value || !source.value.ads) {
      return;
    }
    const extist = source.value.lives.some((i) => i.url == item.url);
    if (!extist) {
      source.value.lives.push(item);
    }
    await done();
  };
  const update_wallpaper = async (wallpaper: string) => {
    if (!source.value) {
      return;
    }
    source.value.wallpaper = wallpaper;
    await done();
  };
  const update_warningText = async (warningText: string) => {
    if (!source.value) {
      return;
    }
    source.value.warningText = warningText;
    await done();
  };
  const update_loading = (b?: boolean) => {
    loading.value = !!b;
  };
  const update_merginSource = async (t: string) => {
    merginSource.value = t;
    await localstorage.set(MERGIN_SOURCE, t);
  };
  const mergin = async () => {
    const src = merginSource.value.trim();
    if (!src) {
      return;
    }
    const lines = src
      .split(/[,;，；\n]/)
      .map((line) => line.trim())
      .filter((line) => {
        return line && /https:\/\//.test(line);
      });
    for (let i = 0; i < lines.length; i++) {
      const item = lines[i];
      await push(item).catch((_) => null);
    }
  };
  const cache = async () => {
    if (!source.value) {
      return;
    }
    const src = JSON.parse(JSON.stringify(source.value)) as TvBoxSource;
    src.sites.forEach((i) => {
      if (!i.jar) {
        i.jar = src.spider;
      }
    });
    await invoke("cache", {
      key: "tvbox",
      value: JSON.stringify(src, undefined, " "),
    });
  };
  return {
    source,
    source_text,
    loading,
    merginSource,
    init,
    load,
    push,
    check,
    check_by,
    check_live,
    check_parses,
    remove_by,
    remove_live,
    remove_parses,
    remove_ads,
    add_live,
    update_wallpaper,
    update_warningText,
    update_loading,
    update_merginSource,
    mergin,
    cache,
  };
});

export { useTvBoxStore };
