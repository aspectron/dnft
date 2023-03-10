function $(selector){
    return document.querySelector(selector)
}
function $dialog(selector){
    let dialog = $(selector)
    if (!dialog.showModal) {
        dialogPolyfill.registerDialog(dialog);
    }
    dialog.querySelector('.close-dialog').addEventListener('click', ()=>{
        dialog.close();
    });
    return dialog
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
        this.initMsgDialog();
        await this.initApp();
        this.initBrowsePage();
        this.initCreateDnftForm();
        this.initMintDnftPage();

        this.setLoading(false);
        this.initUpload();
    }

    initUpload(){
        this.fileInput = document.createElement("input");
        this.fileInput.setAttribute("type", "file");
        this.fileInput.classList.add("hidden-file-input");
        document.body.appendChild(this.fileInput);
        this.fileInput.addEventListener("change", async ()=>{
            let file = this.fileInput.files[0];
            if (!file){
                return
            }
            this.fileInput.value = "";
            let formData = new FormData();
            //formData.append("name", file.name);
            formData.append("file", file);
            fetch('/upload/file', {method: "POST", body: formData})
            .then(res=>res.json())
            .then(data=>{
                console.log("result", data);
                this.fileUploadCallback?.(data);
                this.fileUploadCallback = null;
            })
            .catch(err=>{
                this.fileUploadCallback?.(err);
                this.fileUploadCallback = null;
            })
            
        })
    }

    uploadFile(callback){
        this.fileUploadCallback = callback;
        this.fileInput.click();
    }
    uploadImage(callback){
        this.fileInput.setAttribute("accept", "image/png, image/jpeg, image/svg, image/bmp")
        this.uploadFile(callback);
    }
    showToast(message, actionText=null, actionHandler=null, timeout=10000){
        let data = {
            message,
            timeout,
            actionHandler,
            actionText
        };
        this._toastBar = this._toastBar || $("#toast-bar");
        this._toastBar.MaterialSnackbar.showSnackbar(data);
    }

    initMsgDialog(){
        let dialog = $dialog('#msg-dialog');
        this.msgEl = $('#msg-dialog .msg');
        this.msgTitleEl = $('#msg-dialog .title');
        this.msgDialog = dialog;
    }

    showError(msg){
        this.msgDialog.classList.add("error");
        this.msgTitleEl.innerHTML = "Error";
        this.msgEl.innerHTML = msg;
        this.msgDialog.showModal();
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
            if (event == "transaction-created"){
                if (data?.transaction?.name){
                    this.showToast(data.transaction.name)
                }
                return
            }
            if (event == "transaction-failure"){
                if (data?.error?.includes("Attempt to debit an account but")){
                    this.showError("Attempt to debit an account but found no record of a prior credit.")
                }
                this.dnft.discardTxChain(data.txChain.id)
                return
            }
            if (event != "transaction-success" || !data){
                return 
            }
            this.dnft.discardTxChain(data.txChain.id)
            let name = data.transaction.name.toLowerCase()||"";
            let accounts = data.transaction.meta.accounts;
            if (name.includes("creating mint")){
                this.loadMints();
            }else if (name.includes("creating token")){
                this.loadNFT(accounts[0]);
            }else if (name.includes("updating token") || name.includes("buy token")){
                setTimeout(()=>{
                    this.loadNFT(accounts[0]);
                }, 3000)
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
        let timeoutId = null;
        const observer = new IntersectionObserver( 
            ([e]) => {
                //console.log("e.intersectionRatio", e.intersectionRatio)
                if (timeoutId){
                    clearTimeout(timeoutId);
                }
                timeoutId = setTimeout(()=>{
                    //console.log("Ratio###", e.intersectionRatio)
                    e.target.toggleAttribute('stuck', e.intersectionRatio < 1)
                }, 1)
                
            },
            {
                //root: $("main"),
                threshold: [1]
            }
        );
        observer.observe($('#marketplace-header'));

        // const mainObserver = new IntersectionObserver( 
        //     ([e]) => {
        //         console.log("e.intersectionRatio", e.intersectionRatio)
        //     },
        //     {
        //         threshold: [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1]
        //     }
        // );
        // mainObserver.observe($('#marketplace'));
        
        let connected = await this.dnftApp.checkWalletState();
        if (!connected){
            //$(".wallet-connect-container").classList.remove("connected");
        }
    }

    onWalletConnect(key){
        this.walletPubkey = key;
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
        this.marketNFTTemplateEl = $("#market-panel-tpl");
        this.nftListEl = $("#nft-list");
        this.marketplaceListEl = $("#marketplace-list");
        this.saleSettingDialog = $dialog("#sale-setting");
        this.marketFilter = {};
        this.mainEl = mainEl;
        this.loadMints();
        this.loadNFTs();
        this.loadMarketplace();
        
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
        });

        $$(`.marketplace-header input[name="sale-type"]`).forEach(input=>{
            input.addEventListener("change", (e)=>{
                //e.preventDefault();
                let saleType = "any";
                $$(`input[name="sale-type"]`).forEach(input=>{
                    if(input.checked){
                        saleType = input.value;
                    }
                });

                console.log("saleType:"+saleType)
                this.marketFilter.saleType = saleType!="any"? this.dnft.SaleType.fromStr(saleType) : undefined;
                console.log("this.marketFilter", this.marketFilter)
                this.loadMarketplace();
            })
        });

        [this.marketplaceListEl, this.nftListEl].forEach(list=>{
            list.addEventListener("click", (e)=>{
                let el = e.target.closest("[data-action]");
                let nftEl = el?.closest(".nft-panel");
                if(!nftEl)
                    return
                let action = el.dataset.action;
                let {pubkey, mint} = nftEl.dataset;
                if (!pubkey || !mint)
                    return
                if(action=="buy"){
                    this.dnft.buyToken(pubkey);
                    return
                }

                if(action=="setting"){
                    this.openSaleSetting(pubkey, mint, nftEl.coinMeta);
                    return
                }
            })
        })

        this.saleSettingDialog.querySelector(".update-btn").addEventListener("click", ()=>{
            this.updateSaleSettingBtnCallback?.();
        })
        this.saleSettingDialog.querySelector("#setting-for-sale-input").addEventListener("change", (e)=>{
            this.updateSettingListed(e.target.checked)
        })
    }

    updateSettingListed(listed){
        let checkbox = $("#setting-for-sale").MaterialCheckbox;
        let priceField = $("#setting-sale-price");
        let priceInput = priceField.MaterialTextfield;
        if (listed){
            checkbox.check();
            priceInput.enable();
            priceField.parentElement.classList.remove("disabled")
        }else{
            checkbox.uncheck();
            priceInput.disable();
            priceField.parentElement.classList.add("disabled")
        }

        $$(`[name="setting-sale-type"]`).forEach(input=>{
            //console.log("input.value", checked, input.value, input.parentElement.MaterialRadio)
            if (listed && input.value == "none"){
                input.parentElement.MaterialRadio.enable();
            }else{
                input.parentElement.MaterialRadio.disable();
            }
        })

        
    }

    openSaleSetting(pubkey, mint, coinMeta){
        this.saleSettingDialog.querySelector(".pubkey").innerHTML = pubkey;//this.dnft.shortenPubkey(pubkey)
        let checkbox = $("#setting-for-sale").MaterialCheckbox;
        let sale = coinMeta.sale();
        console.log("sale.listed()", sale.listed());
        console.log("sale.sale_type()", sale.sale_type());
        let em = sale.exchange_mechanics();
        let lastSalePrice = em?this.dnft.lamportsToSol(em.price):"";
        $("#setting-sale-price-input").value = lastSalePrice;
        let lastListed = coinMeta.sale().listed();
        this.updateSettingListed(lastListed)

        this.updateSaleSettingBtnCallback = ()=>{
            this.saleSettingDialog.close();
            let listed = undefined;
            let newListed = !!checkbox.inputElement_.checked;
            if (lastListed != newListed){
                listed = newListed
            }
            let salePrice = $("#setting-sale-price-input").value;
            if (salePrice != lastSalePrice)
                salePrice = +salePrice
            else{
                salePrice = undefined
            }
            this.dnft.updateTokenSaleSetting(pubkey, listed, salePrice);
        }
        
        this.saleSettingDialog.showModal();
    }

    getProgramAccounts(config){
        return this.transport.getProgramAccounts(this.programId, config);
    }

    async loadMarketplace(){
        if (this._marketLoading)
            return
        this._marketLoading = true;
        const LOAD_COUNT = 1000;
        let count = 1000n;
        let filter = this.marketFilter;
        if (!this._marketLoadState){
            this._marketLoadState = {start: 0n, saleType:1};
        }

        let loadState = this._marketLoadState;
        let havePlaceholder = false;
        if (filter.saleType != loadState.saleType){
            havePlaceholder = true;
            this._marketLoadState = {
                forSale: filter.forSale,
                saleType: filter.saleType,
                start: 0n
            }
            loadState = this._marketLoadState;
            //TODO: add placeholders
            let panels = this.createNFTPanels("", {schema:[]}, [
                [],[],[],[],[],[],[],[],[],[]
            ]);
            if (this.mainEl.scrollTop >=150 && this.getActiveTabName() == "marketplace"){
                this.mainEl.scrollTop = 150;
            }
            this.marketplaceListEl.innerHTML = "";
            panels.forEach(panel=>{
                this.marketplaceListEl.appendChild(panel);
            })
        }
        
        
        let {start} = loadState;
        
        let pubkeys = await this.dnft.getMintPubkeys(start, start+count);
        
        console.log("getMintPubkeys: start:", start, "pubkeys:", pubkeys)
        let elements = [];
        for (let mint of pubkeys){
            if (elements.length >= LOAD_COUNT){
                break;
            }
            let minData = await this.dnft.getMintData(mint);
            //console.log("mint data", mint, minData);
            let page = loadState[mint] || 0;
            let accounts = [];
            do{
                if (filter.saleType) {
                    accounts = await this.dnft.getMarketTokensByType(
                        mint,
                        page,
                        filter.saleType
                    );
                }else{
                    accounts = await this.dnft.getMarketTokens(mint, page);
                }                
                console.log("getTokens::::", "page:"+page, accounts);

                let panels = this.createNFTPanels(mint, minData, accounts, this.marketNFTTemplateEl);
                elements.push(...panels);
                loadState[mint] = page
                page++;
            } while (accounts.length && elements.length < LOAD_COUNT);
        }
        let scrollTop = this.mainEl.scrollTop;
        if (havePlaceholder){
            this.marketplaceListEl.innerHTML = "";
        }
        elements.map(el=>this.marketplaceListEl.appendChild(el));
        let length = pubkeys.length;
        if (length){
            loadState.start = start + BigInt(length);
            if (this.getActiveTabName() == "marketplace"){
                this.mainEl.scrollTop = scrollTop;
            }
        }
        this._marketLoading = false;
    }

    async loadNFT(tokenPubkey, loadCount=0){
        let account = await this.dnft.getToken(tokenPubkey)
        .catch(err=>{
            if (loadCount < 60){
                setTimeout(()=>{
                    this.loadNFT(tokenPubkey, loadCount++)
                }, 1000)
            }
        })
        if (!account)
            return
        let mintPubkey = account[1].mint();
        let minData = this.mintData[mintPubkey];
        if (!minData){
            console.error("minData data not found for : "+mintPubkey, this.mintData)
            return
        }
        console.log("loadNFT::::", account);
        const addPanel = (list, tpl)=>{
            let panel = this.createNFTPanel(mintPubkey, minData, ...account, tpl);
            let oldPanel = list.querySelector(`.nft-panel[data-pubkey="${panel.dataset.pubkey}"]`)
            //console.log("panel.dataset", panel, oldPanel)
            if (oldPanel){
                list.insertBefore(panel, oldPanel);
                oldPanel.remove();
            }else{
                list.appendChild(panel);
            }
        }
        //console.log("account[1]?.sale?.()", account[1]?.sale?.().listed())
        if (account[1]?.sale?.().listed()){
            //console.log("adding to marketplace");
            addPanel(this.marketplaceListEl, this.marketNFTTemplateEl)
        }else{
            let oldPanel = this.marketplaceListEl.querySelector(`.nft-panel[data-pubkey="${account[0]}"]`)
            if (oldPanel){
                oldPanel.remove();
            }
        }
        addPanel(this.nftListEl)
    }

    async loadNFTs(){
        if (this._nftsLoading)
            return
        this._nftsLoading = true;
        const LOAD_COUNT = 1000;
        let count = 1000n;
        let start = this._nftStartIndex || 0n;
        let pubkeys = await this.dnft.getMintPubkeys(start, start+count);
        this.mintData = this.mintData || {};
        let loadState = {};
        //console.log("getMintPubkeys: start:", start, "pubkeys:", pubkeys)
        let elements = [];
        for (let mint of pubkeys){
            let minData = await this.dnft.getMintData(mint);
            this.mintData[mint] = minData;
            //console.log("mint data", mint, minData);
            let page = loadState[mint] || 0;
            let accounts;
            do{
                accounts = await this.dnft.getAllTokens(
                    mint,
                    page
                );
                //console.log("getAllTokens::::", accounts);

                let panels = this.createNFTPanels(mint, minData, accounts);
                elements.push(...panels);
                loadState[mint] = page
                page++;
            } while (accounts.length && elements.length < LOAD_COUNT);
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

    createNFTPanels(mint, minData, accounts, tpl){
        return accounts.map(([pubkey, meta, data, account])=>{
            return this.createNFTPanel(mint, minData, pubkey, meta, data, account, tpl);
        })
    }
    createNFTPanel(mint, minData, pubkey, meta, data, account, tpl){
        
        const clone = (tpl || this.nftTemplateEl).content.cloneNode(true);
        let el = clone.children[0];
        if (!pubkey){
            el.classList.add("placeholder-panel");
        }
        //if (account){
        //     console.log("account:" , account, account.key().toString(), account.lamports())
        // }
        //console.log("meta:", meta);
        el.dataset.pubkey = pubkey;
        if (meta){
            //let authority = meta.authority().toString();
            let sale = meta.sale();
            if (sale.listed()){
                el.listed = true;
                let em = sale.exchange_mechanics()
                // console.log("sale.listed", sale.listed());
                // console.log("sale.sale_type", sale.sale_type());
                // console.log("sale.exchange_mechanics", sale.exchange_mechanics());
                if (em?.sale_type == "sale"){
                    let salePriceEl = clone.querySelector(".sale-price");
                    let price = this.dnft.lamportsToSol(em.price).toFixed(5);
                    salePriceEl.innerHTML = `${price} ${em.coin}`
                }

            }
            el.coinMeta = meta;
            if (this.walletPubkey.toString() == meta.authority().toString()){
                el.classList.add("mycoin")
            }
            //el.dataset.authority = authority;
        }
        el.dataset.mint = mint;
        let pubkeyEl = clone.querySelector(".nft-pubkey");
        pubkeyEl.innerHTML = pubkey?this.dnft.shortenPubkey(pubkey):"ABCD....WXYZ";
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

        return el;
        
    }

    async loadMints(){
        if (this._mintAccountsLoading)
            return
        this._mintAccountsLoading = true;
        let count = 1000n;
        let start = this._mintAccountsStartIndex || 0n;
        let pubkeys = await this.dnft.getMintPubkeys(start, start+count);
        let scrollTop = this.mainEl.scrollTop;
        //console.log("getMintPubkeys: start:", start, "pubkeys:", pubkeys)
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
        let pubkey_text = document.createElement("div");
        pubkey_text.setAttribute("class", "mint-pubkey");
        pubkey_text.innerHTML = this.dnft.shortenPubkey(pubkey);
        let td_name_text = document.createElement("div");
        td_name_text.innerHTML = "DNFT "+index;
        td_name.appendChild(pubkey_text);
        td_name.appendChild(td_name_text);
        td_name.setAttribute("class", "mint-name-cell mdl-data-table__cell--non-numeric");

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


        let dialog = $dialog('#add-field-dialog');
        let addFieldButtons = $$('.add-field-btn');

        addFieldButtons.forEach(btn=>btn.addEventListener('click', ()=>{
            //clearSelected();
            dialog.showModal();
        }));

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
        this.mintFormDialog = $dialog("#mint-form-dialog");
        this.mintFormFieldsEl = $("#mint-form-fields");

        this.schemaListEl.addEventListener("click", event=>{
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
                false,
                this.dnft.SaleType.none(),
                fieldsData
            );

            if (result){
                this.mintFormDialog.close();
            }

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
            }else if(["ImageUrl", "PageUrl", "StorageProviderUrl"].includes(type)){
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
            if (type == "ImageUrl"){
                let uploadBtn = document.createElement("a");
                uploadBtn.innerHTML = "Or upload image";
                uploadBtn.setAttribute("class", "mdl-textfield__upload-link");
                uploadBtn.setAttribute("href", "javascript: void 0");
                uploadBtn.addEventListener("click", ()=>{
                    this.uploadImage((result)=>{
                        if (result.success && result.file){
                            input.value = location.origin+"/"+result.file;
                        }
                    })
                });
                fieldEl.appendChild(uploadBtn);
            }
            
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
