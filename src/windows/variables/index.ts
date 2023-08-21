import {emit, once} from '@tauri-apps/api/event';
import {invoke} from '@tauri-apps/api/tauri';

const unlisten = async()=>{
    return new Promise(async (resolve)=>{
        try {
            await once("pre_process_data", async (data)=>{
                if(data!==undefined)resolve(data);
            })
        } catch (e) {
            console.error('Error listening to data: ', e);
        }
    })
}
let data:any;
try{
    data=await unlisten();
    let variables:any=data.payload.data.variables;
    let length: number=data.payload.data.length;
    let value: string=data.payload.data.value;
    const replacements: Record<string, string> = createTable(variables);
    const submit_button = document.getElementById("submit-button");
    const cancel_button = document.getElementById("cancel-button");
    if (!submit_button || !cancel_button) {
        console.error("Button element not found.");
    }
    else{
        submit_button.addEventListener("click", async () => {
            value=replace_variables(replacements,value);
            await invoke('run_backspace',{length: length}).then(async ()=>{
                await invoke('run_string',{value: value}).then(async ()=>{
                    emit('close_window',{});
                });
            });
        });
    }

} catch(e){
    console.error('Error processing data: ', e);
}
function replace_variables(replacements: Record<string, string>,value: string){
    const modifiedReplacements: Record<string, string> = {};
  
  // Modify the keys to include {{ and }} at the beginning and end
  for (const key of Object.keys(replacements)) {
    modifiedReplacements[`{{${key}}}`] = replacements[key];
  }
  
  const regex = new RegExp(Object.keys(modifiedReplacements).join("|"), "g");
  return value.replace(regex, match => modifiedReplacements[match]);
}
function createTable(variables: any): Record<string, string>{
    const tableContainer = document.getElementById("table-container");
    if (!tableContainer) {
        console.error("Table container element not found.");
        return {};
    }
  tableContainer.innerHTML = ''; // Clear previous content

  const replacements: Record<string, string> = {};

  // Create table headers
  const headerRow = document.createElement("tr");

  const leftHeaderCell = document.createElement("th");
  leftHeaderCell.textContent = "Variables";
  headerRow.appendChild(leftHeaderCell);

  const rightHeaderCell = document.createElement("th");
  rightHeaderCell.textContent = "Heading Values";
  headerRow.appendChild(rightHeaderCell);

  tableContainer.appendChild(headerRow);

  for (const item of variables) {
    const row = document.createElement("tr");

    const leftCell = document.createElement("td");
    leftCell.textContent = item;
    row.appendChild(leftCell);

    const inputCell = document.createElement("td");
    const inputBox = document.createElement("input");
    inputBox.type = "text";
    inputCell.appendChild(inputBox);
    row.appendChild(inputCell);

    inputBox.addEventListener("input", (event) => {
      const inputValue = (event.target as HTMLInputElement).value;
      replacements[item] = inputValue;
    });

    tableContainer.appendChild(row);
  }
  let container = document.getElementById("variable-container");
    if (!container) {
        console.error("Variable container element not found.");
        return {};
    }
    container.innerHTML = ''; // Clear previous content
    container.appendChild(tableContainer);
    return replacements;
}