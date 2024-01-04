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
  }).catch((e) => {
    console.log(e);
    return [];
  });
  console.timeEnd("check");
  return value;
}
async function try_play(url: string) {
  const exist = await invoke("is_install", { application: "mpv" });
  if (!exist) {
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
function to_playlist(content: string) {
  return content
    .split("\n")
    .map((line) => {
      const item = line.split(",", 2);
      return {
        name: item[0],
        url: item[1] || "",
        online: 0,
        http: maybe_http(item[1]),
      };
    })
    .filter((item) => item.name);
}

function maybe_http(str: string) {
  if (!str) {
    return false;
  }
  return /^https?:\/\//i.test(str);
}

function dedup_by_url(items: TxtPlaylist[]) {
  const urls = items.filter((item) => item.http).map((item) => item.url);
  let keys = Array.from(new Set(urls));
  const list = items.filter((item) => {
    if (!item.http) {
      return true;
    }
    if (keys.includes(item.url)) {
      keys = keys.filter((k) => k != item.url);
      return true;
    }
    return false;
  });
  return list;
}

const useTxtPlaylistStore = defineStore("txt_playlist", () => {
  const content = ref([] as TxtPlaylist[]);
  const content_text = computed(() => {
    return to_string();
  });
  const load = async (uri: string) => {
    const text = await loadResource(uri);
    const list = to_playlist(text);
    const items = dedup_by_url(list);
    content.value = items;
  };
  const push = async (uri: string) => {
    const text = await loadResource(uri);
    const items = to_playlist(text);
    const new_items = content.value.concat(items);
    content.value = dedup_by_url(new_items);
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
  const update = (text: string) => {
    const list = to_playlist(text);
    const items = dedup_by_url(list);
    content.value = items;
  };
  const cache = async () => {
    await invoke("cache", { key: "playlist", value: content_text.value });
  };
  return {
    content,
    content_text,
    load,
    push,
    to_string,
    check,
    check_by,
    remove_by,
    play,
    update,
    cache,
  };
});

export { useTxtPlaylistStore };
