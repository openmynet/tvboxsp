interface TvBoxSource {
  sites: TvBoxVod[];
  lives: TvBoxLive[];
  parses?: TvBoxParse[];
  flags?: string[];
  ijk: TvBoxIjk[];
  rules?: TvBoxRule[];
  ads?: string[];
  wallpaper?: string;
  spider?: string;
  warningText?: string;
}
interface TvBoxVod {
  key: string;
  name: string;
  type: number;
  api: string;
  searchable: number;
  quickSearch: number;
  filterable?: number;
  ext?: any;
  // 自定义JAR刮削库 - 引用第三方
  jar?: string;
  /// 播放器类型<br>
  ///  0 system 1 ikj 2 exo 10 mxplayer -1 以参数设置页面的为准
  player_type?: number;
  /// 分类&排序
  categories?: string[];
  /// 需要点击播放的嗅探站点selector   ddrk.me;#id
  click?: string;
  hide?: number;

  features?: string;
  // 0:To be tested  1: valid, -1: invalid
  status?: number;
}

interface TvBoxLive {
  name?: string;
  group?: string;
  channels?: {
    name: string;
    urls: string[];
  }[];
  epg?: string;
  type?: number;
  url?: string;
  full_url?: string;
  // 0:To be tested  1: valid, -1: invalid
  status?: number;
}

interface TvBoxParse {
  name: string;
  type: number;
  url: string;
  ext?: any;
  // 0:To be tested  1: valid, -1: invalid
  status?: number;
}

interface TvBoxIjk {
  group: string;
  options: {
    category: number;
    name: string;
    value: string;
  }[];
}
interface TvBoxRule {
  hosts?: string[];
  name?: string;
  regex?: string[];
  host?: string;
  rule?: string[];
}

interface Playlist {
  loss: number;
  count: number;
  content: string;
}

interface TxtPlaylist {
  name: string;
  url: string;
  // 0: uncheck, -1, loss, 1: online
  online?: number;
  http?: boolean;
}

interface ConnectionStatus {
  connectable: boolean;
  extra: any;
}
