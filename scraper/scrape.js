// Import puppeteer
const puppeteer = require('puppeteer');
const fs = require('fs');
const shell = require('shelljs');

(async () => {

  let start_page = 1;
  fs.readFile('place.txt', 'utf8', function (err, data) {
    // Display the file content
    console.log(data);
    start_page = parseInt(data);
  });


  // Launch the browser
  const browser = await puppeteer.launch({
    headless: true, args: ['--start-maximized',
      `--disable-blink-features=AutomationControlled`,
      `--disable-web-security`,
      `--allow-running-insecure-content`],
    defaultViewport: null
  });

  // Create a page
  const page = await browser.newPage();
  await page.setUserAgent(
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36'
  );
  // Go to your site
  await page.goto('https://www.ncbi.nlm.nih.gov/pmc/?term=ayurveda');
  await page.setViewport({width: 1080, height: 1024});
  // await page.waitForNavigation();
  // await page.type('#term', 'ayurveda');
  // Evaluate JavaScript
  // set page to 100 terms
  await page.evaluate(() => {
    document.getElementsByClassName("items")[1].childNodes[1].childNodes[4].childNodes[0].click();
    document.getElementById("pageno").focus();
  });

  await page.waitForNavigation({WaitForOptions:'load'});
  // for(var pageno=0; pageno<start_page; pageno++){
  //   console.log("Skipping page...\nCurrent Page: "+pageno);
  //   const searchResultSelector = '.next';
  //   page.locator(searchResultSelector).click();
  //   await page.waitForNavigation();
  // }
  // await page.focus('#pageno');
  if(start_page>1){
  await page.locator('#pageno').fill(""+start_page);
  await page.evaluate(() => {
    document.getElementById("pageno").focus();
  });
  await page.keyboard.press('Enter');
  await page.waitForNavigation({WaitForOptions:'load'});
}

  const real_page_num = await page.evaluate(()=>{
    return document.getElementById("pageno").value;
  });
  console.log(real_page_num);

  console.log("Starting download");
  // await page.waitForNavigation();
  for(var pageno=start_page;pageno<=188;pageno++){
    console.log("current page: "+pageno);
    // if(pageno<start_page){
    //   console.log("Skipping!");
    //   const searchResultSelector = '.next';
    //   await page.waitForSelector(searchResultSelector);
    //   await page.click(searchResultSelector);
    //   await page.waitForNavigation();
    //   continue;
    // }

    const data = await page.evaluate(() => {
      console.log("Evaluating!!")
      let entries = document.getElementsByClassName("rslt");
      let entry_list = [];
      function text_or_none(element){
        if(element!=null&&element.innerText!=null){
          return element.innerText
        }else{
          return "None";
        }
      }
      console.log("starting loop...");
      for(var element of entries){

        let title = text_or_none(element.childNodes[0]);
        let authors_and_dates = element.childNodes[1].childNodes[0].innerText;
        let PMCID = text_or_none(element.childNodes[2]);
        let abstract = "None";
        let article = "None";
        let pdf = "None";

        let linkNodes = element.childNodes[3].childNodes[0].childNodes;
        for (link of linkNodes){
          console.log(link)
          if (link.innerText=="Abstract"){
            abstract = link.href
          }else if(link.innerText=="Article"){
            article = link.href;
          }else if(link.innerText.includes("PDF")){
            pdf = link.href;
          }
        }


        let json_inator = {
          title:title,
          publication_info:authors_and_dates,
          PMCID: PMCID,
          abstract_link:abstract,
          article_link:article,
          pdf_link:pdf
        };
        console.log(json_inator);
        console.log("pushing!")
        entry_list.push(json_inator);
      }
      console.log("completed loop");
      return JSON.stringify(entry_list);
    });
    // console.log(data);
    let temp_objs = JSON.parse(data);
    // let article_license = "Unknown";
    let total_data = [];
    // console.log(total_data);
    for(var temp_obj of temp_objs){
      if (temp_obj.abstract_link!="None" && temp_obj.abstract_link!=null){
        const new_temp_page = await browser.newPage();
        await new_temp_page.setUserAgent(
          'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36'
        );
        console.log("Going to: "+ temp_obj.abstract_link);
        await new_temp_page.goto(temp_obj.abstract_link);
        await new_temp_page.setViewport({width: 1080, height: 1024});
        const retracted = await new_temp_page.evaluate(()=>{
          if (document.getElementsByClassName("retraction-alert").length>0){
            return true;
          }else{
            return false;
          }
        });
        if (retracted){
          //skip page
          continue;
        }
        const abstract = await new_temp_page.evaluate(()=>{
          if (document.getElementsByClassName("abstract").length>0){
            return document.getElementsByClassName("abstract")[0].innerText.split("\n");
          }else{
            console.log("No Abstract FOUND!!!!");
            return "None";
          }
        });

        const license = await new_temp_page.evaluate(()=>{
          if (document.getElementsByClassName("license").length>0){
            return document.getElementsByClassName("license")[0].innerText;
          }else{
            return "Unknown";
          }
        });
        const doi = await new_temp_page.evaluate(()=>{
          if(document.getElementsByClassName("usa-link usa-link--external")[0]!=null){
            return document.getElementsByClassName("usa-link usa-link--external")[0].innerText;
          }else{
            return "Unknown";
          }
        });
        const keywords = await new_temp_page.evaluate(()=>{
          if(document.getElementsByClassName("kwd-group")[0]!=null){
            return document.getElementsByClassName("kwd-group")[0].innerText.replace("Keywords: ","").split(",").map((x)=>x.trim());
          }else{
            return ["None"]
          }
        });
        const author_data = await new_temp_page.evaluate(()=>{
          let author_data = [];
          let id = 1;
          while(document.getElementById("id"+id)!=null){
            let author = document.getElementById("id"+id);
            let name = author.childNodes[1].innerText;
            let relations = [];
            let start = 3;
            while (!author.childNodes[start].innerText.includes("Find")){
              relations.push(author.childNodes[start].innerText);
              start+=2;
            }
            let current_author = {
              name: name,
              attributes: relations,
              email:"random@sweep.rs"
            };
            author_data.push(current_author);
            id++;
          }
          return author_data;
        });

        temp_obj.abstract_text = abstract;
        temp_obj.license_agreement = license;
        temp_obj.keywords = keywords;
        temp_obj.doi = doi;
        temp_obj.publication_info = author_data;
        //possible correspondence cats = corr1, c1-jcm, contrib-email, single email - single author,
        //Proccess - find how many authors there are
        //if only one, attach the fm-affl data to it
        //search for oemail class, reverse the email, and attach it to the author

        //If there is more than one author, grab fm author, parse it to find sups
        new_temp_page.close();
      }else{
        temp_obj.abstract_text = "None";
        temp_obj.license_agreement = "None";
        temp_obj.keywords = "None";
        temp_obj.doi = "None";
        // temp_obj.publication_info = author_data;
      }
      total_data.push(temp_obj);
    }
    console.log(total_data);
    let jsonify = JSON.stringify(total_data);
    // download(data, "scrapedata.json", "application/json");
    console.log("Storing");
    let file_name = "page"+pageno+".json";
    fs.writeFile(file_name, jsonify, err => {
      if (err) throw err;
    });

    fs.writeFile("place.txt", ""+(pageno+1), err => {
      if (err) throw err;
    });
    shell.exec('./mullvad_reload.sh')
    
    console.log("Nexting")
    const searchResultSelector = '.next';
    await page.waitForSelector(searchResultSelector);
    await page.click(searchResultSelector);
    await page.waitForNavigation();
  }

  // Close browser.
  await browser.close();
})();


