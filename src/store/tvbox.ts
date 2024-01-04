import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
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
  const source = ref(undefined as undefined | TvBoxSource);
  const source_text = computed(() => {
    return JSON.stringify(source, undefined, "");
  });
  const loading = ref(false);
  const load = async (uri: string) => {
    loading.value = true;
    const value = await loadResource(uri);
    source.value = value;
    loading.value = false;
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
    loading.value = false;
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
    loading.value = false;
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
    loading.value = false;
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
    loading.value = false;
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
    loading.value = false;
  };
  const remove_by = (items: TvBoxVod[]) => {
    if (!source.value) {
      return;
    }
    const keys = items.map((item) => item.name + "-" + item.key);
    source.value.sites = source.value?.sites.filter((item) => {
      const key = item.name + "-" + item.key;
      return !keys.includes(key);
    });
  };
  const remove_live = (i: number) => {
    if (!source.value) {
      return;
    }
    source.value.lives.splice(i, 1);
  };
  const remove_parses = (i: number | TvBoxParse[]) => {
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
  };
  const remove_ads = (i: number) => {
    if (!source.value || !source.value.ads) {
      return;
    }
    source.value.ads.splice(i, 1);
  };
  const add_live = (item: TvBoxLive) => {
    if (!source.value || !source.value.ads) {
      return;
    }
    const extist = source.value.lives.some((i) => i.url == item.url);
    if (!extist) {
      source.value.lives.push(item);
    }
  };
  const update_wallpaper = (wallpaper: string) => {
    if (!source.value) {
      return;
    }
    source.value.wallpaper = wallpaper;
  };
  const update_warningText = (warningText: string) => {
    if (!source.value) {
      return;
    }
    source.value.warningText = warningText;
  };
  const update_loading = (b?: boolean) => {
    loading.value = !!b;
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
    cache,
  };
});

export { useTvBoxStore };
