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

function createIconBtn(icon, title="", attributes={}){
    let iconEl = document.createElement("i");
    iconEl.setAttribute("class", "material-icons");
    iconEl.innerHTML = icon;

    let btn = document.createElement("button");
    Object.keys(attributes).forEach(key=>{
        btn.setAttribute(key, attributes[key]);
    });
    btn.classList.add("mdl-button", "mdl-button--icon");
    btn.appendChild(iconEl);
    if(title){
        btn.setAttribute("title", title)
    }

    return btn;
}

function createCheckbox(value, label="", id="", cls=""){
    let input = document.createElement('input');
    input.type = "checkbox";
    if(id)
        input.id = id;
    input.setAttribute("class", "mdl-checkbox__input "+cls);
    input.setAttribute("value", value);
    let checkbox = document.createElement('label');
    checkbox.setAttribute("class", "mdl-checkbox mdl-js-checkbox");
    checkbox.appendChild(input);
    if (label){
        let span = document.createElement('span');
        span.setAttribute("class", "mdl-checkbox__label");
        span.innerHTML = label;
        checkbox.appendChild(span);
    }
    componentHandler.upgradeElement(checkbox, "MaterialCheckbox");

    return checkbox;
}

const field_info = {
    min:{
        u8: 0,
        u16: 0,
        u32: 0,
        u64: 0,
        u128: 0,
        i8: -(2**7),
        i16: -(2**15),
        i32: -(2**31),
        i64: BigInt(-(2**63)),
        i128: BigInt(-(2**127)),
        f32: -3.40282347E+38,
        f32_positive: 1.17549435e-38,
        f64: -1.7976931348623157e+308,
        f64_positive: 2.2250738585072014e-308
    },
    max:{
        u8: (2**8)-1,
        u16: (2**16)-1,
        u32: (2**32)-1,
        u64: BigInt((2**64)-1),
        u128: BigInt((2**128)-1),
        i8: (2**7)-1,
        i16: (2**15)-1,
        i32: (2**31)-1,
        i64: BigInt((2**63)-1),
        i128: BigInt((2**127)-1),
        f32: 3.40282347e+38,
        f64: 1.7976931348623157e+308
    }
};

console.log("field_info", field_info)


class App{

    constructor(dnft){
        this.dnft = dnft;
        this.init();
    }

    init(){
        this.initCreateDnftForm();
        this.initMintDnftPage();
    }

    initCreateDnftForm(){
        this.fieldListEl = $("#field-list");
        this.fieldTypeListEl = $("#field-type-list");
        this.createDnftMintBtn = $("#create-dnft-mint-btn");

        const { Field, DataType, Data } = this.dnft;

        let fields = [];
        let dataTypes = Object.keys(DataType).filter(k => !isFinite(+k));
        for (let dataType of dataTypes) {
            //let name = ("field-"+dataType).toLowerCase();
            let descr = `Descr for ${dataType}`;

            fields.push(new Field(DataType[dataType], dataType, descr));
        }

        for(let field of fields){
            let type = field.dataType()
            let checkbox = createCheckbox(type, "", "checkbox-field-type-"+type, "field-type");
            
            let td_checkbox = document.createElement("td");
            td_checkbox.appendChild(checkbox);

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

        this.fieldListEl.addEventListener("click", event=>{
            let tr = event.target.closest("tr");
            let actionEl = event.target.closest("[data-action]");
            if(!actionEl || !tr)
                return
            let action = actionEl.dataset.action;
            switch(action){
                case "delete":
                    tr.remove();
                break;
                case "move-up":
                    tr.parentElement.insertBefore(tr, tr.previousElementSibling)
                break;
                case "move-down":
                    tr.parentElement.insertBefore(tr, tr.nextElementSibling.nextElementSibling)
                break;
            }
        });

        this.createDnftMintBtn.addEventListener("click", async ()=>{
            let trList = this.fieldListEl.querySelectorAll("tr");
            let fields = [];
            trList.forEach(tr=>{
                let dataType = tr.children[0].innerText;
                let name = escapeHtml(tr.children[1].innerText).replace(/\n/g, " ")
                let discription = escapeHtml(tr.children[2].innerText).replace(/\n/g, " ");
                //let a = DataType[dataType];
                //console.log("dataType:", dataType, a);
                fields.push(new Field(DataType[dataType], name, discription))
            })

            console.log("fields[0]", fields[0].dataType(), fields[0].name(), fields[0].description())
            let schema = new this.dnft.Schema(fields)
            let pubkey = await this.dnft.createDnftMint(schema);
            console.log("create_dnft_mint: result", pubkey);

            this.loadSchema(pubkey);
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

    async loadSchema(pubkey){
        let schema = await this.dnft.loadSchema(pubkey);
        fields = schema.fields();
        console.log("schema: result", fields);

        this.buildMintForm(fields);
        this.activateMintForm();
    }

    initMintDnftPage(){
        let schemaListEl = $("#schema-list");
        this.schemaListPanel = $("#schema-list-panel");
        this.mintFormPanel = $("#mint-form-panel");
        this.mintFormFieldsEl = $("#mint-form-fields");

        schemaListEl.addEventListener("click", event=>{
            let el = event.target.closest(".schema-item");
            let btn = event.target.closest("button.mint-dnft");
            if(!btn || !el)
                return
            
            //TODO get schema/fields
            const { Field, DataType, Data } = this.dnft;

            let fields = [];
            let dataTypes = Object.keys(DataType).filter(k => !isFinite(+k));
            for (let dataType of dataTypes) {
                let name = "Field "+dataType;
                let descr = `Descr for ${dataType}`;

                fields.push(new Field(DataType[dataType], name, descr));
            }

            this.buildMintForm(fields);
            this.activateMintForm();
            
        })
    }

    activateTab(tab){
        
    }

    activateMintForm(){
        this.mintFormPanel.classList.add("is-active");
        this.schemaListPanel.classList.remove("is-active");
    }

    buildMintForm(fields){
        while(this.mintFormFieldsEl.childNodes.length){
            this.mintFormFieldsEl.childNodes[0].remove();
        }
        for(let field of fields){
            let el = this.createFormField(field);
            this.mintFormFieldsEl.appendChild(el);
        }
    }

    createFormField(field, attributes={}){
        let type = this.dnft.DataType[field.dataType()];

        console.log("createFormField: type", type )
        let createField = ()=>{
            if (type == "Bool"){
                let checkbox = createCheckbox("ON", field.name());
                return checkbox
            }
        
            let input = document.createElement("input");
            input.setAttribute("class", "mdl-textfield__input");
            input.type = "text";
            
           if(["u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64"].includes(type)){
                input.type = "number";
                input.min = field_info.min[type];
                input.max = field_info.max[type];
            }else if(type == "ImageUrl" || type == "PageUrl"){
                input.type = "url";
            }
            
            let label = document.createElement("label");
            label.setAttribute("class", "mdl-textfield__label");
            label.innerHTML = field.name();

            let error = document.createElement("span");
            error.setAttribute("class", "mdl-textfield__error");
            error.innerHTML = "Invalid value";
        
            let fieldEl = document.createElement("div");
            Object.keys(attributes).forEach(key=>{
                fieldEl.setAttribute(key, attributes[key]);
            });
        
            fieldEl.classList.add("mdl-textfield", "mdl-textfield--floating-label", "mdl-js-textfield", "has-placeholder");
            fieldEl.appendChild(input);
            fieldEl.appendChild(label);
            fieldEl.appendChild(error);
            
            componentHandler.upgradeElement(fieldEl, "MaterialTextfield");

            if(["u64", "u128", "i64", "i128"/*, "f32", "f64"*/].includes(type)){
                fieldEl.MaterialTextfield.checkValidity = () =>{
                    let isValid = BigInt(input.value) >= BigInt(field_info.min[type]) &&
                    BigInt(input.value) <= BigInt(field_info.max[type])

                    //console.log("isValid:", type, input.value, isValid)

                    if (isValid) {
                        fieldEl.classList.remove(fieldEl.MaterialTextfield.CssClasses_.IS_INVALID);
                    } else {
                        fieldEl.classList.add(fieldEl.MaterialTextfield.CssClasses_.IS_INVALID);
                    }
                }
            }

            return fieldEl;
        }

        let fieldEl = createField();
        let info = document.createElement("div");
        info.setAttribute("class", "form-field__info-text");
        info.innerHTML = field.description();
        //let infoIcon = document.createElement("i");
        //infoIcon.setAttribute("class", "material-icons");
        //infoIcon.innerHTML = "info";

        let infoBox = document.createElement("div");
        infoBox.setAttribute("class", "form-field__info");
        //infoBox.appendChild(infoIcon)
        infoBox.appendChild(info)

        let formField = document.createElement("div");
        formField.setAttribute("class", "form-field--with-info");
        formField.appendChild(fieldEl);
        formField.appendChild(infoBox);
        
        return formField;
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


            let btn_move_down = createIconBtn("expand_more", "Move down", {"data-action":"move-down"});
            let btn_move_up = createIconBtn("expand_less", "Move up", {"data-action":"move-up"});
            let btn_delete = createIconBtn("delete", "Delete", {"data-action":"delete"});

            let td_action = document.createElement("td");
            td_action.appendChild(btn_move_down);
            td_action.appendChild(btn_move_up);
            td_action.appendChild(btn_delete);
            td_action.setAttribute("class", "actions");

            let tr = document.createElement("tr");
            tr.appendChild(td_type);
            tr.appendChild(td_name);
            tr.appendChild(td_descr);
            tr.appendChild(td_action);
            this.fieldListEl.appendChild(tr);
        }
    }
}
