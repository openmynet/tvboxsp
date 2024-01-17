import { Store } from "tauri-plugin-store-api";
const datastore = new Store(".settings.dat");

export default datastore;
