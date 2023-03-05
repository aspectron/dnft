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
    checkbox.input = input;
    if (label){
        let span = document.createElement('span');
        span.setAttribute("class", "mdl-checkbox__label");
        span.innerHTML = label;
        checkbox.appendChild(span);
    }
    componentHandler.upgradeElement(checkbox, "MaterialCheckbox");

    return checkbox;
}
const UnsignedNumberFields = ["u8", "u16", "u32", "u64", "u128"];
const SignedNumberFields = ["i8", "i16", "i32", "i64", "i128"];
const FloatingNumberFields = ["f32", "f64"];
const NumberFields = [
    ...UnsignedNumberFields,
    ...SignedNumberFields,
    ...FloatingNumberFields
]

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

    constructor(dnft, transport){
        this.dnft = dnft;
        this.transport = transport;
        this.programId = dnft.dnft_program_id();
        this.init();

        window._app = this;
    }

    async init(){
        await this.initApp();
        this.initBrowsePage();
        this.initCreateDnftForm();
        this.initMintDnftPage();

        this.setLoading(false);
    }

    async initApp(){
        this.dnftApp = await new this.dnft.Application("dnft-store-name");
        this.dnftApp.onWalletConnect(this.onWalletConnect.bind(this));

        let layoutEl = $(".mdl-js-layout");
        layoutEl.addEventListener("mdl-componentupgraded", this.afterLayoutReady.bind(this));
        if (layoutEl.classList.contains("is-upgraded")){
            await this.afterLayoutReady();
        }

        let connectBtn = $("#wallet-connect");
        connectBtn.addEventListener("click", async ()=>{
            await this.dnftApp.connectWallet();
        });

        this.txObserver = new this.dnft.TransactionObserver();
        this.txObserver.setHandler(({event, data})=>{
            console.log("txObserver:", event, data);
            if (event != "transaction-success" || !data)
                return 
            let name = data.transaction.name.toLowerCase()||"";
            let accounts = data.transaction.meta.accounts;
            if (name.includes("creating mint")){
                this.loadMints();
            }else if (name.includes("creating token")){
                this.loadNFT(accounts[1], accounts[0]);
            }
        })

        /*
        this.reflectorClient = new this.dnft.ReflectorClient()

        this.reflectorClient.setHandler((e)=>{
            console.log("reflectorClient: handler:", e);
        })
        this.reflectorClient.start();
        */
    }

    async afterLayoutReady(e){
        if (this._layoutReady)
            return
        this._layoutReady = true;
        
        let connected = await this.dnftApp.checkWalletState();
        if (!connected){
            //$(".wallet-connect-container").classList.remove("connected");
        }
    }

    onWalletConnect(key){
        console.log("wallet-connected ::: pubkey: ", key.toString());
        $("#wallet-pubkey").innerHTML = this.dnft.shortenPubkey(key.toString());
        $(".wallet-connect-container").classList.add("connected");
    }

    setLoading(loading){
        if(loading){
            document.body.classList.add("loading")
        }else{
            document.body.classList.remove("loading")
        }
    }

    async initBrowsePage(){
        let mainEl = $("main");
        this.schemaListEl = $("#schema-list");
        this.nftTemplateEl = $("#nft-panel-tpl");
        this.nftListEl = $("#nft-list");
        this.mainEl = mainEl;
        this.loadMints();
        this.loadNFTs();

        
        let browseEl = $("#browse");
        let browseMintsEl = $("#browse-mints");
        let footerEl = $(".mdl-mega-footer");
        mainEl.addEventListener("scroll", (event)=>{
            let isBrowseActive = browseEl.classList.contains("is-active");
            let isBrowseMintsActive = browseMintsEl.classList.contains("is-active");
            if(!isBrowseActive && !isBrowseMintsActive)
                return;
            
            let contentHeight = mainEl.scrollHeight - footerEl.offsetHeight;
            let scrolled = mainEl.scrollTop + mainEl.offsetHeight;
            //let height_90 = contentHeight*0.9;
            let margin = 500;
            /*
            console.log(
                "contentHeight", contentHeight, 
                "offsetHeight", mainEl.offsetHeight, 
                "scrollTop", mainEl.scrollTop , 
                "scrolled", scrolled, 
                //"height_90", height_90,
                //scrolled>height_90
            )
            */
            if (scrolled>contentHeight-margin){
                if (isBrowseActive){
                    this.loadNFTs();
                }else{
                    this.loadMints();
                }
            }
        })
    }

    getProgramAccounts(config){
        return this.transport.getProgramAccounts(this.programId, config);
    }

    async loadNFT(mintPubkey, tokenPubkey){
        let minData = await this.dnft.getMintData(mintPubkey);
        let account = await this.dnft.getToken(tokenPubkey);
        console.log("getToken::::", account);

        let panel = this.createNFTPanel(mintPubkey, minData, ...account);
        this.nftListEl.appendChild(panel);
    }

    async loadNFTs(){
        if (this._nftsLoading)
            return
        this._nftsLoading = true;
        let count = 5n;
        let start = this._nftStartIndex || 0n;
        let pubkeys = await this.dnft.getMintPubkeys(start, start+count);
        
        console.log("getMintPubkeys: start:", start, "pubkeys:", pubkeys)
        let index = start+1n;
        let elements = [];
        for (let mint of pubkeys){
            let minData = await this.dnft.getMintData(mint);
            console.log("mint data", mint, minData);

            let accounts = await this.dnft.getTokens(mint, 0);
            console.log("getProgramAccounts::::", accounts);

            let panels = this.createNFTPanels(index++, mint, minData, accounts);
            elements.push(...panels);
        }
        let scrollTop = this.mainEl.scrollTop;
        elements.map(el=>this.nftListEl.appendChild(el));
        let length = pubkeys.length;
        if (length){
            this._nftStartIndex = start + BigInt(length);
            if (this.getActiveTabName() == "browse"){
                this.mainEl.scrollTop = scrollTop;
            }
        }
        this._nftsLoading = false;
    }

    createNFTPanels(index, mint, minData, accounts){
        return accounts.map(([pubkey, data, account])=>{
            return this.createNFTPanel(mint, minData, pubkey, data, account);
        })
    }
    createNFTPanel(mint, minData, pubkey, data, account){
        const clone = this.nftTemplateEl.content.cloneNode(true);
        let el = clone.children[0];
        el.dataset.pubkey = pubkey;
        el.dataset.mint = mint;
        let title = clone.querySelector(".nft-title");
        title.setAttribute("title", pubkey);
        title.innerHTML = "&nbsp;";
        let img = clone.querySelector(".nft-image");
        let description = clone.querySelector(".nft-description");
        minData.schema.forEach((field, index)=>{
            let el = document.createElement("div");
            let value = data[index];
            //console.log("index, type, value", index, field.type, value)
            if (field.type == "ImageUrl"){
                img.style.backgroundImage = `url(${value})`;
            }
            el.innerHTML = `<label>${field.name}: </label> <span>${value||""}</span>`;
            description.appendChild(el);
            if (field.name == "Name" && typeof value == "string"){
                title.textContent = value;
            }
        });

        return clone;
        
    }

    async loadMints(){
        if (this._mintAccountsLoading)
            return
        this._mintAccountsLoading = true;
        let count = 5n;
        let start = this._mintAccountsStartIndex || 0n;
        let pubkeys = await this.dnft.getMintPubkeys(start, start+count);
        let scrollTop = this.mainEl.scrollTop;
        console.log("getMintPubkeys: start:", start, "pubkeys:", pubkeys)
        let index = start+1n;
        for (let pubkey of pubkeys){
            let data = await this.dnft.getMintData(pubkey);
            //console.log("pubkey data", pubkey, data);
            let el = this.createMintRow(index++, pubkey, data);
            this.schemaListEl.appendChild(el);
        }
        let length = pubkeys.length;
        if (length){
            this._mintAccountsStartIndex = start + BigInt(length);
            if (this.getActiveTabName() == "browse-mints"){
                this.mainEl.scrollTop = scrollTop;
            }
        }
        this._mintAccountsLoading = false;
    }

    createMintRow(index, pubkey, data){
        let td_name = document.createElement("td");
        td_name.innerHTML = "DNFT "+index;
        td_name.setAttribute("class", "mdl-data-table__cell--non-numeric");

        let td_description = document.createElement("td");
        let description = ["<bold>Fields</bold>"];
        for (let field of data.schema){
            description.push(`${field.type}: ${field.name}, ${field.description}`)
        }
        td_description.innerHTML = `<p>${description.join("<br />")}</p>`;
        td_description.setAttribute("class", "mdl-data-table__cell--wrap-text");

        
        let btn = document.createElement("button");
        btn.classList.add("mdl-button", "mint-dnft");
        btn.innerHTML = "Mint it";
        btn.dataset.pubkey = pubkey;

        let td_action = document.createElement("td");
        td_action.appendChild(btn);

        let tr = document.createElement("tr");
        tr.appendChild(td_name);
        tr.appendChild(td_description);
        tr.appendChild(td_action);

        return tr;
    }

    initCreateDnftForm(){
        this.fieldListEl = $("#field-list");
        this.fieldTypeListEl = $("#field-type-list");
        this.createDnftMintBtn = $("#create-dnft-mint-btn");

        const { Field, DataType, Data } = this.dnft;

        let fields = [];
        let dataTypes = Object.keys(DataType).filter(k => !isFinite(+k));
        for (let dataType of dataTypes) {
            let descr = `Descr for ${dataType}`;

            fields.push(new Field(DataType[dataType], dataType, descr));
        }

        for(let field of fields){
            let type = field.dataType()

            let td_type = document.createElement("td");
            td_type.innerHTML = field.name();
            td_type.setAttribute("class", "mdl-data-table__cell--non-numeric");

            let td_descr = document.createElement("td");
            td_descr.innerHTML = field.description();
            td_descr.setAttribute("class", "mdl-data-table__cell--non-numeric");

            let tr = document.createElement("tr");
            tr.setAttribute("data-type", type);
            tr.appendChild(td_type);
            tr.appendChild(td_descr);
            this.fieldTypeListEl.appendChild(tr);
        }

        this.fieldTypeListEl.addEventListener("click", (event)=>{
            let label = event.target.closest("label")
            if (label)
                return

            let tr = event.target.closest("tr[data-type]")
            if(!tr)
                return
            let type = tr.dataset.type;

            dialog.close();
            let fields = [type].map(dataType=>{
                return new Field(+dataType, "", "")
            });

            if (fields.length){
                $("#create-dnft-main-container").classList.remove("no-fields");
            }

            this.appendToFieldList(fields);
        });


        let dialog = $('#add-field-dialog');
        let addFieldButtons = $$('.add-field-btn');
        if (!dialog.showModal) {
            dialogPolyfill.registerDialog(dialog);
        }

        addFieldButtons.forEach(btn=>btn.addEventListener('click', ()=>{
            //clearSelected();
            dialog.showModal();
        }));

        dialog.querySelector('.close-dialog').addEventListener('click', ()=>{
            dialog.close();
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
            let ids = await this.dnft.createMint(schema)
            .catch(err=>{
                console.log("Unable to create MINT: ", err);
            })

            if(ids){
                console.log("createMint: result", ids);
            }
        })

    }

    async loadSchema(pubkey){
        let mintData = await this.dnft.getMintData(pubkey);
        this.buildMintForm(mintData.schema);
        this.mintFormDialog._mintPubkey = pubkey;
    }

    initMintDnftPage(){
        let schemaListEl = $("#schema-list");
        this.mintFormDialog = $("#mint-form-dialog");
        this.mintFormFieldsEl = $("#mint-form-fields");
        if (!this.mintFormDialog.showModal) {
            dialogPolyfill.registerDialog(this.mintFormDialog);
        }

        this.mintFormDialog.querySelector('.close-dialog').addEventListener('click', ()=>{
            this.mintFormDialog.close();
        });

        schemaListEl.addEventListener("click", event=>{
            let btn = event.target.closest("button.mint-dnft");
            if(!btn)
                return
            
            this.loadSchema(btn.dataset.pubkey);
            this.mintFormDialog.showModal();
        });

        const createData = (field, value)=>{
            let index = this.dnft.DataType[field.type];
            //console.log("value:", field, index, value, Data)
            if (NumberFields.includes(field.type)){
                value = +value;
            }
            return new this.dnft.Data(index, value);
        }

        $("#mint-dnft-btn").addEventListener("click", async ()=>{
            let inputs = this.mintFormFieldsEl.querySelectorAll(".mdl-textfield__input");
            //const { Field, DataType, Data } = this.dnft;
            let fieldsData = [];
            inputs.forEach(input=>{
                let data = createData(input._field, input.value);
                console.log("data:", data);
                fieldsData.push(data);
            });
            let mintPubkey = this.mintFormDialog._mintPubkey;
            let result = await this.dnft.createToken(
                mintPubkey,
                true,
                this.dnft.SaleType.rent(),
                fieldsData
            );

            console.log("mint result:", result);
        })
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
        let type = field.type;

        console.log("createFormField: type", type)
        let createField = ()=>{
            if (type == "Bool"){
                let checkbox = createCheckbox("ON", field.name);
                checkbox.input._field = field;
                return checkbox
            }
        
            let input = document.createElement("input");
            input.setAttribute("class", "mdl-textfield__input");
            input._field = field;
            input.type = "text";
            
           if(NumberFields.includes(type)){
                input.type = "number";
                input.min = field_info.min[type];
                input.max = field_info.max[type];
            }else if(type == "Url"){
                input.type = "url";
            }
            
            let label = document.createElement("label");
            label.setAttribute("class", "mdl-textfield__label");
            label.innerHTML = field.name;

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
        info.innerHTML = field.description;
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


            let input_name = document.createElement("div");
            input_name.innerHTML = field.name();
            input_name.setAttribute("class", "editable");
            input_name.setAttribute("contentEditable", "true");

            let td_name = document.createElement("td");
            td_name.appendChild(input_name);
            td_name.setAttribute("class", "mdl-data-table__cell--non-numeric edit-container");


            let input_descr = document.createElement("div");
            input_descr.innerHTML = field.description();
            input_descr.setAttribute("class", "editable");
            input_descr.setAttribute("contentEditable", "true");

            let td_descr = document.createElement("td");
            td_descr.appendChild(input_descr);
            td_descr.setAttribute("class", "mdl-data-table__cell--non-numeric edit-container");


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

    activateTab(tab){
        let tabEl = $(`#top-tabs [href='#${tab}']`);
        tabEl?.show();
        $(`main`).scrollTo({
            top: 0,
            behavior: "smooth"
        });
    }

    getActiveTab(){
        return $(`#top-tabs .mdl-layout__tab.is-active`);
    }
    getActiveTabName(){
        let tabEl = this.getActiveTab();
        if(!tabEl)
            return "";
        return (tabEl.getAttribute("href")??"").replace("#", "");
    }
}
