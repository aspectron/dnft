function $(selector){
    return document.querySelector(selector)
}

function $$(selector){
    return document.querySelectorAll(selector)
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
        const { Field, DataType, Data } = this.dnft;

        console.log("DataType::", DataType);

        let fields = [];
        let dataTypes = Object.keys(DataType).filter(k => !isFinite(+k));
        for (let dataType of dataTypes) {
            let name = ("field-"+dataType).toLowerCase();
            let descr = `Descr for ${dataType}`;

            fields.push(new Field(DataType[dataType], name, descr));
        }
        

        let select_field = $("#datatype")

        for(let field of fields){
            //console.log("field", field)
            //console.log("vec", field.name(), field.dataType(), field.description());
            let option = document.createElement("option");
            option.setAttribute("value", field.dataType());
            option.innerHTML = field.name();
            select_field.appendChild(option);
        }

        select_field.classList.add("is-dirty");
        select_field.value = 0;


        var dialog = $('#add-field-dialog');
        var addFieldButton = $('#add-field-btn');
        if (!dialog.showModal) {
          dialogPolyfill.registerDialog(dialog);
        }

        addFieldButton.addEventListener('click', function() {
          dialog.showModal();
        });

        dialog.querySelector('.close').addEventListener('click', function() {
          dialog.close();
        });

    }
}
