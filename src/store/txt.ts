import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { dirname, basename, sep } from "@tauri-apps/api/path";
import { confirm } from "../utils";
async function loadResource(uri: string) {
  uri = uri.trim();
  if (!uri) {
    return "";
  }
  const content = await invoke<string>("get_content", {
    uri,
  }).catch((e) => {
    console.log(e);
    return "";
  });
  return content;
}
async function checkResource(items: TxtPlaylist[]) {
  if (!items.length) {
    return [];
  }
  console.time("check");
  const value = await invoke<string[]>("urls_accessibility", {
    urls: items.map((item) => item.url),
    check_m3u8: true,
  }).catch((e) => {
    console.log(e);
    return [];
  });
  console.timeEnd("check");
  return value;
}
async function try_play(url: string) {
  const exist = await invoke("is_install", { application: "mpv" });
  if (exist) {
    await invoke("exec", { args: `start mpv ${url}` });
    return;
  }
  let exec = window.localStorage.getItem("native_player");
  if (!exec) {
    await confirm("请配置mpv播放器");
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "mpv",
          extensions: ["exe", ""],
        },
      ],
    });
    if (Array.isArray(selected)) {
      exec = selected[0];
    } else if (typeof selected == "string") {
      exec = selected;
    }
    window.localStorage.setItem("native_player", exec || "");
  }
  if (!exec) {
    return;
  }
  const dir = await dirname(exec);
  const dir_fixed = dir
    .split(sep)
    .map((name) => {
      if (/\s/.test(name)) {
        return `"${name}"`;
      }
      return name;
    })
    .join(sep);
  const name = await basename(exec);
  const args = `start /d ${dir_fixed} ${name.split(".")[0]} ${url}`;
  // TODO 仅对windows做了处理
  const info = await invoke("exec", { args }).catch((e) => {
    console.log("exec.error", e);
  });
  console.log("exec.info", info);
}
async function to_playlist(content: string) {
  const items = content
    .split("\n")
    .map((line) => {
      const item = line.split(",", 2);
      return {
        name: item[0],
        url: item[1] || "",
        online: 0,
        http: maybe_http(item[1]),
        group: !item[1] || item[1].startsWith("#"),
        raw: line,
        hash: ""
      } as TxtPlaylist;
    })
    .filter((item) => item.url);
  for (let i = 0; i < items.length; i++) {
    items[i].hash = await invoke<string>("hash", {
      content: items[i].raw,
    })
  }

  return items;
}

function maybe_http(str: string) {
  if (!str) {
    return false;
  }
  return /^https?:\/\//i.test(str);
}

function dedup_by_url(items: TxtPlaylist[]) {
  const hashs = [] as string[];
  let list = [] as TxtPlaylist[];
  items.forEach((item) => {
    if (!hashs.includes(item.hash)) {
      hashs.push(item.hash);
      list.push(item)
    }
  })
  return list;
}

const useTxtPlaylistStore = defineStore("txt_playlist", () => {
  const content = ref([] as TxtPlaylist[]);
  const content_text = computed(() => {
    return to_string();
  });
  const group = computed({
    get() {
      let i = 0;
      let items = [] as ITxtPlaylistGroup[];
      content.value.forEach((item: TxtPlaylist) => {
        if (item.group) {
          i = items.length;
          if (!items[i]) {
            items[i] = { group: item.name, raw: item, items: [] } as ITxtPlaylistGroup;
          }
        } else {
          items[i].items.push(item);
        }
      });
      return items;
    },
    set(v: ITxtPlaylistGroup[]) {
      const items = v.map(i => {
        return [i.raw, ...i.items]
      }).flat();
      content.value = items;
    }
  })
  const load = async (uri: string) => {
    const text = await loadResource(uri);
    const list = await to_playlist(text);
    const items = dedup_by_url(list);
    content.value = items;
  };
  const push = async (uri: string) => {
    const text = await loadResource(uri);
    const items = await to_playlist(text);
    const new_items = content.value.concat(items);
    const deduped = dedup_by_url(new_items);
    // 除去空白分组
    let index = 0;
    let list = deduped.filter((item, i) => {
      if (!item.url) {
        const current = index + 1;
        index = i;
        if (current == i) {
          return false;
        }
      }
      return true
    })
    content.value = list;
  };
  const to_string = () => {
    return content.value
      .map((item) => {
        return `${item.name},${item.url}`;
      })
      .join("\n");
  };
  const check = async () => {
    const urls = await checkResource(content.value);
    content.value.forEach((item) => {
      if (urls.includes(item.url)) {
        item.online = 1;
      } else if (item.http) {
        item.online = -1;
      }
    });
  };
  const check_by = async (items: TxtPlaylist[]) => {
    const keys = items.map((item) => item.url);
    const urls = await checkResource(items);
    content.value.forEach((item) => {
      if (keys.includes(item.url)) {
        item.online = urls.includes(item.url) ? 1 : -1;
      }
    });
  };
  const remove_by = (items: TxtPlaylist[]) => {
    const keys = items.map((item) => item.name + "-" + item.url);
    content.value = content.value.filter((item) => {
      const key = item.name + "-" + item.url;
      return !keys.includes(key);
    });
  };
  const play = async (item: TxtPlaylist) => {
    if (item.http) {
      try_play(item.url);
    }
  };
  const update = async (text: string) => {
    const list = await to_playlist(text);
    const items = dedup_by_url(list);
    content.value = items;
  };
  const cache = async () => {
    await invoke("cache", { key: "playlist", value: content_text.value });
  };
  const group_move = async (hashs: string[], from: string, to: string) => {
    console.log('hash', hashs);
    
    let retain = [] as TxtPlaylist[];
    const items = group.value.map(item => {
      if (item.raw.hash == from) {
        retain = item.items.filter(i => hashs.includes(i.hash));
        console.log(retain.length);
        
        item.items = item.items.filter(i => !hashs.includes(i.hash))
        return item
      } else {
        return item
      }
    })
    console.log('retain', retain.length);
    
    items.forEach(item => {
      if (item.raw.hash == to) {
        console.log('retain', retain.length, to);
        item.items.push(...retain);
      }
    })
    group.value = items;
  }
  return {
    content,
    content_text,
    group,
    load,
    push,
    to_string,
    check,
    check_by,
    remove_by,
    play,
    update,
    cache,
    group_move,
  };
});

export { useTxtPlaylistStore };
