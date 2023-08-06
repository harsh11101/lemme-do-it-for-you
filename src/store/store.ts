import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri'

interface KeyValueData {
  trigger: string;
  value: string;
}

export const userInfo = defineStore('user', {
  state: () => ({
    dataFromServer: new Array<KeyValueData>(),
  }),
  getters: {
    getData: (state) => state.dataFromServer,
  },
  actions: {
    async refreshData() {
        await this.getDataFromServer();
    },
    async getDataFromServer() {
      await invoke('get_all_data').then((res:any) => {
        let data =new Array<KeyValueData>();
        Array.from(res).forEach((element: any) => {
            data.push({trigger: element.trigger, value: element.value});
        });
        this.dataFromServer = data;
      });
    },
    async deleteObjectWithKey(trigger:string) {
      await invoke('delete_by_trigger', {trigger: trigger}).then(async () => {
        await this.getDataFromServer();
      });
    },
    async updateDataWithId(trigger:string, value:string) {
        await invoke('update_by_trigger', {trigger: trigger, value: value}).then(async () => {
            await this.getDataFromServer();
        });
    },
    async deleteAllData() {
        await invoke('delete_all_data').then(async () => {
            await this.getDataFromServer();
        });
    },
    async addDataInServer(trigger: string, value: string) {
        Array.from(this.dataFromServer).forEach((element: any) => {
            if(element.trigger.toString().startsWith(trigger)) {
                throw new Error("Trigger already exists");
            }
        });
        await invoke('insert_data', {trigger: trigger.toString(), value: value.toString()}).then(() => {
            this.getDataFromServer();
        });
    },
  },
});