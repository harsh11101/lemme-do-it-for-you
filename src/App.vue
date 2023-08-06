<template>
  <div class="container">
    <h2>Your Triggers</h2>
    <div id="no-data" v-if="infoStore.dataFromServer.length===0">No data found. Please add data to be shown here.</div>
    <div class="snippets" v-else>
      <div v-for="elements in infoStore.dataFromServer" :key="elements.trigger" class="ind-data" :id="elements.trigger">
        <div contenteditable="true" style="display: inline;" class="left data-holder data-key" @click="store_current_trigger(elements.trigger)">{{elements.trigger}}</div>
        <div contenteditable="true" style="display: inline;" class="middle data-holder data-value" @click="store_current_trigger(elements.trigger)">{{elements.value}}</div>
        <button @click="deleteThisObject(elements.trigger)" class="right data-holder delete-button action">Delete</button>
        <button @click="updateData($event,elements.trigger)" class="right data-holder update-button">Update</button>
    </div>
    </div>
    <div class="add-new-data">
      <h3>Add new Trigger:</h3>
      <div class="form-to-add">
        <form v-on:submit.prevent class="add-new-form">
          <input type="text" placeholder="Enter key" v-model="key_add" class="input-data" id="input-key" required>
          <input type="text" placeholder="Enter value" v-model="value_add" class="input-data" id="input-value" required>
          <button type="submit" @click="addData(key_add,value_add)" id="add-btn" class="action">
            Add
          </button>
        </form>
      </div>
    </div>
    <div class="action-buttons bottom-most">
      <button class="action" id="refresh-action" @click="refresh_values">Refresh</button>
      <button class="action" id="delete-all-action" @click="delete_all_values">Delete All</button>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onBeforeMount } from 'vue';
  import { userInfo } from './store/store.ts';
  const infoStore=userInfo();
  onBeforeMount(async ()=>{
      await infoStore.getDataFromServer();
  })
  let key_add = "";
  let value_add = "";
  let prev_trigger="";
  async function addData(key_add:string,value_add:string){
    if(key_add==="" || value_add==="")return;
    await infoStore.addDataInServer(key_add,value_add);
    key_add="";
    value_add="";
    (document.getElementById("input-key") as HTMLInputElement).value = "";
    (document.getElementById("input-value") as HTMLInputElement).value = "";
    location.reload();
  }
  function refresh_values(){
    location.reload();
    prev_trigger="";
    key_add="";
    value_add="";
  }
  async function delete_all_values(){
    await infoStore.deleteAllData();
  }
  async function deleteThisObject(trigger:string){
    await infoStore.deleteObjectWithKey(trigger);
  }
  async function updateData(event:any,trigger:string){
    if(prev_trigger==="" || prev_trigger!==trigger)return;
    await infoStore.deleteObjectWithKey(prev_trigger);
    prev_trigger="";
    await infoStore.addDataInServer(event.target.parentElement.children[0].innerText,event.target.parentElement.children[1].innerText);
  }
  function store_current_trigger(trigger:string){
    prev_trigger=trigger;
  }
</script>

<style>
::placeholder{
  color: white;
  opacity: 0.65;
}
.container {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  background-color: black;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
h2,h3{
  color: white;
}
#no-data{
  color: white;
  opacity: 0.5;
}
.add-new-data{
  text-align: left;
}
.add-new-form{
  display: flex;
  justify-content: space-between;
}
.input-data{
  margin:0px 3px 3px 3px;
  padding:4px 4px 4px 10px;
  color: white;
  border: white 2px solid;
  border-radius: 8px;
}
#input-key{
  background-color: black;
}
#input-value{
  background-color: black;
}
#add-btn{
  background-color: white;
  border: black 2px solid;
  color: black;
}
.action{
  margin:3px 10px 3px 3px;
  border-radius: 10px;
  padding:6px 20px 5px 20px;
  background-color: white;
  border: black 2px solid;
  color: black;
}
.ind-data{
  margin: 3px 3px 3px 3px;
  display: flex;
  justify-content: space-around;
  padding:4px 10px 4px 10px;
}
.data-holder{
  margin: 3px 3px 3px 3px;
  padding:4px 4px 4px 10px;
  background-color: black;
  border: white 2px solid;
  color: white;
  border-radius: 10px;
  width: 200px;
  text-align: left;
}
.right{
  width:100px;
  text-align: center;
  margin:3px 3px 3px 3px;
  border-radius: 10px;
  padding:6px 20px 5px 20px;
  background-color: white;
  border: black 2px solid;
  color: black;
  max-height: 30px;
}
.middle{
  margin-right: 20px;
}
</style>
