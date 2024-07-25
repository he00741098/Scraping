// Import puppeteer
const puppeteer = require('puppeteer');
const fs = require('fs');
(async () => {
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
    console.log("Set to 100 things");
  });

  let start_page = 8;
  for(var pageno=0; pageno<start_page; pageno++){
    console.log("Skipping page...\nCurrent Page: "+pageno);
    const searchResultSelector = '.next';
    // await page.waitForSelector(searchResultSelector);
    // await page.click(searchResultSelector);
    await page.waitForNavigation();
    page.locator(searchResultSelector).click();
    // if (pageno<6){
    // }
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
        const abstract = await new_temp_page.evaluate(()=>{
          if (document.getElementsByClassName("tsec sec")[0].childNodes[3]!=null){
            return document.getElementsByClassName("tsec sec")[0].childNodes[3].innerText.split("\n");
          }else if(document.getElementsByClassName("tsec sec")[0].childNodes[2]!=null){
            return document.getElementsByClassName("tsec sec")[0].childNodes[2].innerText.split("\n");
          }else{
            console.log("No Abstract FOUND!!!!");
            return "None";
          }
        });
        const license = await new_temp_page.evaluate(()=>{
          if (document.getElementsByClassName("license")[0]!=null){
            return document.getElementsByClassName("license")[0].innerText;
          }else{
            return "Unknown";
          }
        });
        const doi = await new_temp_page.evaluate(()=>{
          if(document.getElementsByClassName("doi")[0]!=null){
            if(document.getElementsByClassName("doi")[0].childNodes[1]!=null){
              return document.getElementsByClassName("doi")[0].childNodes[1].innerText;
            }else{
              return "Unknown";
            }
          }else{
            return "Unknown";
          }
        });
        const keywords = await new_temp_page.evaluate(()=>{
          if(document.getElementsByClassName("kwd-text")[0]!=null){
            return document.getElementsByClassName("kwd-text")[0].innerText.split(",").map((x)=>x.trim());
          }else{
            return ["None"]
          }
        });
        const author_data = await new_temp_page.evaluate(()=>{
          let author_list = [];
          let author_data = [];
          let email_list = [];
          let atags = document.getElementsByClassName("oemail");
          for(atag of atags){
            email_list.push(atag.innerText.split("").reverse().join(""));
          }
          function find_email(target){
            target = target.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLowerCase();
            let targets = target.split(" ");
            for(var g of email_list){
              console.log(g);
              if (g.includes(targets[0])){
                return g;
              }else if(targets.length>1&&g.includes(targets[targets.length-1])){
                return g;
              }
            }
            return null;
          }
          document.getElementsByClassName("fm-author")[0].childNodes.forEach((x)=>author_list.push(x.innerText));
          author_list = author_list.filter((f)=>{return f!=null});
          author_list = author_list.filter((f)=>{return f.length>0});
          author_list = author_list.map((f)=>{return f.replace(",", "")});
          if(author_list.length==1){
            //do the thing...
            let author_email = "";
            if(email_list.length==1){
              author_email = email_list[0]
            }else if(email_list.length==0){
              author_email="None";
            }else{
              //try finding email with first name
              author_email = find_email(author_list[0]);
            }
            if (document.getElementsByClassName("fm-affl")[0]!=null){
              author_data.push({
                name:author_list[0],
                attributes:[document.getElementsByClassName("fm-affl")[0].innerText],
                email:author_email
              });
            }else{
              author_data.push({
                name:author_list[0],
                attributes:[],
                email:author_email
              });
            }

          }
          else{

            let dat = document.getElementsByClassName("fm-affl");
            let dat2 = [];
            for(var f of dat){
              if(f.childNodes.length>1){
                dat2[f.childNodes[0].innerText] = f.childNodes[1].textContent;
              }else{
                console.log("No sups");
                dat2[""] = f.innerText;
              }
            }
            let current_author = {
              name:"",
              attributes:[]
            };
            for (var i = 0; i<author_list.length;i++){
              if(author_list[i].length>3){
                if(i>0){
                  author_data.push(current_author);
                  current_author = {
                    name:"",
                    attributes:[]
                  };
                }
                current_author.name = author_list[i];
                current_author.email = find_email(author_list[i]);
              }else{
                let attribute = dat2[author_list[i]];
                if(attribute!=undefined){
                  current_author.attributes.push(attribute);
                }
              }
            }
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

    
    console.log("Nexting")
    const searchResultSelector = '.next';
    await page.waitForSelector(searchResultSelector);
    await page.click(searchResultSelector);
    await page.waitForNavigation();
  }

  // Close browser.
  await browser.close();
})();


