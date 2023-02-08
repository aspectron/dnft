function $(selector){
    return document.querySelector(selector)
}

function $$(selector){
    return document.querySelectorAll(selector)
}
function escapeHtml(text){
    return text
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
 }

class App{

    constructor(dnft){
        this.dnft = dnft;
        this.init();
    }

    init(){
        this.init_create_dnft_form();
    }

    init_create_dnft_form(){
        this.fieldListEl = $("#field-list");
        this.fieldTypeListEl = $("#field-type-list");
        this.createDnftMintBtn = $("#create-dnft-min-btn");

        const { Field, DataType, Data } = this.dnft;

        let fields = [];
        let dataTypes = Object.keys(DataType).filter(k => !isFinite(+k));
        for (let dataType of dataTypes) {
            let name = ("field-"+dataType).toLowerCase();
            let descr = `Descr for ${dataType}`;

            fields.push(new Field(DataType[dataType], dataType, descr));
        }

        for(let field of fields){
            let input = document.createElement('input');
            input.type = "checkbox";
            input.id = "checkbox-field-type-"+field.dataType();
            input.setAttribute("class", "mdl-checkbox__input field-type");
            input.setAttribute("value", field.dataType());
            let label = document.createElement('label');
            label.setAttribute("class", "mdl-checkbox mdl-js-checkbox");
            label.appendChild(input);
            componentHandler.upgradeElement(label);
            
            let td_checkbox = document.createElement("td");
            td_checkbox.appendChild(label);

            let td_type = document.createElement("td");
            td_type.innerHTML = field.name();
            td_type.setAttribute("class", "mdl-data-table__cell--non-numeric");

            let td_descr = document.createElement("td");
            td_descr.innerHTML = field.description();
            td_descr.setAttribute("class", "mdl-data-table__cell--non-numeric");

            let tr = document.createElement("tr");
            tr.appendChild(td_checkbox);
            tr.appendChild(td_type);
            tr.appendChild(td_descr);
            this.fieldTypeListEl.appendChild(tr);
        }

        this.fieldTypeListEl.addEventListener("click", (event)=>{
            let label = event.target.closest("label")
            if (label)
                return

            let tr = event.target.closest("tr")
            if(!tr)
                return

            let checkbox = tr.querySelector("input.field-type");
            if (!checkbox)
                return
            if (checkbox.checked){
                checkbox.parentElement.MaterialCheckbox.uncheck()
            }else{
                checkbox.parentElement.MaterialCheckbox.check()
            }
        });


        let dialog = $('#add-field-dialog');
        let addFieldButtons = $$('.add-field-btn');
        if (!dialog.showModal) {
            dialogPolyfill.registerDialog(dialog);
        }

        addFieldButtons.forEach(btn=>btn.addEventListener('click', ()=>{
            clearSelected();
            dialog.showModal();
        }));

        dialog.querySelector('.close-dialog').addEventListener('click', ()=>{
            dialog.close();
        });

        let getSelected = ()=>{
            let list = [];
            this.fieldTypeListEl.querySelectorAll("input.field-type:checked").forEach(checkbox=>{
                if (checkbox.checked){
                    let dataType = +checkbox.value
                    if (DataType[dataType]){
                        list.push(dataType)
                    }
                }
            });

            return list;
        }
        let clearSelected = ()=>{
            this.fieldTypeListEl.querySelectorAll("input.field-type:checked").forEach(checkbox=>{
                checkbox.parentElement.MaterialCheckbox.uncheck()
            });
        }

        dialog.querySelector('.add-fields').addEventListener('click', ()=>{
            let list = getSelected();
            if (list.length){
                dialog.close();
                let fields = list.map(dataType=>{
                    return new Field(dataType, "", "")
                });

                if (fields.length){
                    $("#create-dnft-main-container").classList.remove("no-fields");
                }

                this.appendToFieldList(fields);
            }
        });

        this.createDnftMintBtn.addEventListener("click", ()=>{
            let trList = this.fieldListEl.querySelectorAll("tr");
            let fields = [];
            trList.forEach(tr=>{
                let dataType = tr.children[0].innerText;
                let name = escapeHtml(tr.children[1].innerText).replace(/\n/g, " ")
                let discription = escapeHtml(tr.children[2].innerText).replace(/\n/g, " ");

                fields.push(new Field(DataType[dataType], name, discription))
            })

            console.log("fields[0]", fields[0].dataType(), fields[0].name(), fields[0].description())
        })
        

        /*
        let floatingInputBox = document.createElement("input");
        floatingInputBox.setAttribute("class", "edit-box__input");
        this.floatingInputBox = floatingInputBox;
        this.floatingInputBox.addEventListener("keypress", (event)=>{
            console.log("event.key", event)
            if (event.key == "Enter"){
                this.floatingInputBox.saveValue?.(this.floatingInputBox.value);
                this.floatingInputBox.value = "";
            }
        })

        this.fieldListEl.addEventListener("click", (event)=>{
            let editBox = event.target.closest(".edit-box");
            if (!editBox){
                return
            }

            if(this.floatingInputBox.parentElement == editBox)
                return
            this.floatingInputBox.value = editBox.innerText;
            this.floatingInputBox.saveValue = (value)=>{
                editBox.innerText = value;
            }

            editBox.appendChild(this.floatingInputBox)
            this.floatingInputBox.focus();
        })
        */
    }

    appendToFieldList(fields){
        const { Field, DataType, Data } = this.dnft;
        for(let field of fields){
            let td_type = document.createElement("td");
            td_type.innerHTML = DataType[field.dataType()];
            td_type.setAttribute("class", "mdl-data-table__cell--non-numeric");

            let td_name = document.createElement("td");
            td_name.innerHTML = field.name();
            td_name.setAttribute("class", "mdl-data-table__cell--non-numeric");
            td_name.setAttribute("contentEditable", "true");

            let td_descr = document.createElement("td");
            td_descr.innerHTML = field.description();
            td_descr.setAttribute("class", "mdl-data-table__cell--non-numeric");
            td_descr.setAttribute("contentEditable", "true");

            let tr = document.createElement("tr");
            tr.appendChild(td_type);
            tr.appendChild(td_name);
            tr.appendChild(td_descr);
            this.fieldListEl.appendChild(tr);
        }
    }
}
