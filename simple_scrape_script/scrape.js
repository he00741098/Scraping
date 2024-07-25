let entries = document.getElementsByClassName("rslt");
let entry_list = [];
for(var element of entries){
  let title = element.childNodes[0].innerText;
  let authors_and_dates = element.childNodes[1].innerText;
  let PMCID = element.childNodes[2].innerText;
  let abstract = element.childNodes[3].childNodes[0].childNodes[0];
  if (abstract!=null){
    abstract = abstract.href;
  }else{
    abstract="None";
  }
  let article = element.childNodes[3].childNodes[0].childNodes[1];
  if (article!=null){
    article= article.href;
  }else{
    article="None";
  }
  let pdf = element.childNodes[3].childNodes[0].childNodes[2];
  if (pdf!=null){
    pdf = pdf.href;
  }else{
    pdf="None";
  }
  let json_inator = {
    title:title,
    publication_info:authors_and_dates,
    PMCID: PMCID,
    abstract_link:abstract,
    article_link:article,
    pdf_link:pdf
  };
  entry_list.push(json_inator);
}

let stringify = JSON.stringify(entry_list);
download(stringify, "scrapedata.json", "application/json");
function download(data, filename, type) {
  var file = new Blob([data], {type: type});
  if (window.navigator.msSaveOrOpenBlob) // IE10+
  window.navigator.msSaveOrOpenBlob(file, filename);
  else { // Others
    var a = document.createElement("a"),
    url = URL.createObjectURL(file);
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    setTimeout(function() {
      document.body.removeChild(a);
      window.URL.revokeObjectURL(url);  
    }, 0); 
  }
}
